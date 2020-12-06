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
        Err(_) => {None}
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
