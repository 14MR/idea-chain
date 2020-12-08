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

    // let transport = web3::transports::Http::new("https://ropsten.infura.io/v3/29428009f85c4773b84275eb5bc68d57");
    // let web3 = web3::Web3::new(transport.unwrap());
    //
    // println!("Calling accounts.");
    // let mut accounts = web3.eth().accounts().then();
    // println!("Accounts: {:?}", accounts);
    // accounts.push("0x077CA1590D6cf5222c92151c1a965C39ce08290B".parse().unwrap());
    //
    // println!("Calling balance.");
    // for account in accounts {
    //     let balance = web3.eth().balance(account, None).poll().wait();
    //     println!("Balance of {:?}: {}", account, balance);
    // }

    //
    // println!("Calling accounts.");
    // let accounts = async move {
    //     let mut accounts = web3.eth().accounts().await.unwrap();
    //     println!("Accounts: {:?}", accounts);
    //     accounts.push("0x077CA1590D6cf5222c92151c1a965C39ce08290B".parse().unwrap());
    //
    //     println!("Calling balance.");
    //     for account in accounts {
    //         let balance = web3.eth().balance(account, None).await;
    //         println!("Balance of {:?}: {}", account, balance.unwrap());
    //     }
    //
    // };



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

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, register])
}
