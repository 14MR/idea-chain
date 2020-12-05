#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use patent_app::establish_connection;

extern crate diesel;
extern crate patent_app;

use self::patent_app::*;
use self::models::*;
use self::diesel::prelude::*;
use patent_app::schema::*;
use rocket::response::content;

#[get("/")]
fn index() -> content::Json<String> {
    use patent_app::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users.filter(id.eq(1))
        .limit(1)
        .load::<User>(&connection)
        .expect("Error loading users");

    content::Json(results)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
