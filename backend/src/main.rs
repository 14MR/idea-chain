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

#[derive(Deserialize, Serialize)]
struct Signature {
    message: String,
    signature: String
}


#[post("/auth", format = "json", data="<signature>")]
async fn auth(signature: Json<Signature>) -> rocket::response::content::Json<String>{
    use web3::types;
    use hex::decode;
    let web3 = eth::init_web3();

    let res = serde_json::to_string(&signature.into_inner());
    match res {
        Ok(res) => {
            let u = serde_json::from_str::<Signature>(&res).unwrap();
            println!("{}", u.signature);
            println!("{}", u.signature.as_bytes().len());

            //let web3 = eth::init_web3();
            let c = web3::types::Recovery::from_raw_signature(u.message, hex::decode(u.signature).unwrap());
            match c{
                Ok(recovery) => {
                    let address = web3.accounts().recover(recovery).unwrap();
                    println!("{}", address.to_string());
                    let b = web3.eth().balance(address, None).await.unwrap();
                    content::Json(b.to_string())
                    // match address {
                    //     None => {content::Json("nothing".to_string())}
                    //     Some(address) => {content::Json(address.().to_string())}
                    // }
                }
                Err(err) => {content::Json(err.to_string())}
            }
        }
        Err(_) => { content::Json("{}".to_string()) }
    }
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, register, auth])
}
