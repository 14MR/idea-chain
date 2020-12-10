use patent_app::schema::users::dsl::*;
use patent_app::schema::*;
// use crate::schema::users;
use patent_app::establish_connection;
use patent_app::models::User;
use diesel::{insert_into, RunQueryDsl, QueryResult};
use rocket::response::content;
use patent_app::schema::users::dsl::users;
use diesel::result::Error;

pub fn create_user(eth_address: String) ->  Option<i32> {
    let connection = establish_connection();

    let u = User {
        id: None,
        eth_key: eth_address,
        first_name: None,
        last_name: None,
    };

    let query_result:QueryResult<User> = insert_into(users).values(&u).get_result(&connection);

    query_result.unwrap().id

}
