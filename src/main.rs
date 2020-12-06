#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use self::establish_connection;

extern crate patent_app;

use self::patent_app::*;
use self::models::*;
use self::schema::*;
use ::diesel::prelude::*;
use rocket::response::content;
use serde_json::Error;

#[get("/")]
fn index() -> Option<content::Json<String>> {
    use patent_app::schema::users::dsl::*;

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

fn main() {
    rocket::ignite().mount("/", routes![index, register]).launch();
}
