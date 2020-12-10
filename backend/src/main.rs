#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use self::establish_connection;

extern crate patent_app;

use self::patent_app::*;
use self::models::*;
use self::schema::*;
use ::diesel::prelude::*;
use rocket::response::content;
use serde_json::Error;
use web3;
mod eth;

#[get("/")]
async fn index() -> Option<content::Json<String>> {
    use patent_app::schema::users::dsl::*;

    let web3 = eth::init_web3();

    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await.unwrap();
    accounts.push("077CA1590D6cf5222c92151c1a965C39ce08290B".parse().unwrap());
    println!("Accounts: {:?}", accounts);


    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await.unwrap();
        println!("Balance of {:?}: {}", account, balance);
    }

    let connection = establish_connection();
    let results = users.filter(id.eq(1))
        .limit(1)
        .load::<User>(&connection)
        .expect("Error loading users");

    let res = serde_json::to_string(&results);
    match res {
        Ok(res) => { Some(content::Json(res)) }
        Err(_) => { None }
    }
}

use rocket_contrib::json::Json;
use diesel::insert_into;
use std::borrow::Borrow;
use web3::futures::FutureExt;

#[post("/register", format = "json", data = "<user>")]
fn register(user: Json<User>) -> Option<rocket::response::content::Json<String>> {
    use schema::users::dsl::*;
    let connection = establish_connection();

    let res = serde_json::to_string(&user.into_inner());
    match res {
        Ok(res) => {
            let u = serde_json::from_str::<User>(&res);

            insert_into(users).values(&u.unwrap()).execute(&connection);

            Some(content::Json(res))
        }
        Err(_) => { None }
    }
}
use serde::*;
use web3::types::{Recovery, RecoveryMessage};
use rocket_cors::{CorsOptions, AllowedOrigins, AllowedHeaders};
use rocket::http::Method;

#[derive(Deserialize, Serialize)]
struct Signature {
    message: String,
    signature: String
}

use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: i32,
    address: String
}

#[derive(Deserialize, Serialize)]
struct TokenResponse {
    token: String
}

#[post("/auth", format = "json", data="<signature>")]
async fn auth(signature: Json<Signature>) -> rocket::response::content::Json<String>{
    use web3::types;
    use hex::decode;
    let web3 = eth::init_web3();

    let res = serde_json::to_string(&signature.into_inner());
    match res {
        Ok(res) => {
            let received_signature = serde_json::from_str::<Signature>(&res).unwrap();
            let recovery_result = web3::types::Recovery::from_raw_signature(received_signature.message, hex::decode(received_signature.signature).unwrap());
            match recovery_result {
                Ok(recovery) => {
                    let address = web3.accounts().recover(recovery).unwrap();

                    let my_claims = Claims {
                        id: 12,
                        address: address.to_string()
                    };

                    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
                    let token_response = TokenResponse {
                        token
                    };
                    content::Json(serde_json::to_string(&token_response).unwrap())
                }
                Err(err) => {content::Json(err.to_string())}
            }
        }
        Err(_) => { content::Json("{}".to_string()) }
    }
}

#[launch]
fn rocket() -> rocket::Rocket {
    let allowed_origins = AllowedOrigins::all();

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors().unwrap();


    rocket::ignite().attach(cors).mount("/", routes![index, register, auth])
}
