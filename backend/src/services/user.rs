use patent_app::schema::users::dsl::*;
use patent_app::schema::*;
// use crate::schema::users;
use patent_app::establish_connection;
use patent_app::models::User;
use diesel::{insert_into, RunQueryDsl, QueryResult, ExpressionMethods};
use rocket::response::content;
use patent_app::schema::users::dsl::users;
use diesel::result::Error;
use diesel::query_dsl::filter_dsl::FilterDsl;

pub fn get_or_create_user(eth_address: String) -> Option<i32> {
    let connection = establish_connection();

    let u = User {
        id: None,
        eth_key: eth_address,
        first_name: None,
        last_name: None,
    };

    let query_result: QueryResult<User> = insert_into(users).values(&u).get_result(&connection);
    match query_result {
        Ok(_) => { query_result.unwrap().id }
        Err(_) => {
            // TODO: rewrite to Result
            // if the user already exists, return it

            let results = users.filter(eth_key.eq(u.eth_key))
                .load::<User>(&connection)
                .expect("Error loading users");

            if results.len() > 0 {
               results[0].id
            } else{
                Some(-1)
            }
        }
    }
}
