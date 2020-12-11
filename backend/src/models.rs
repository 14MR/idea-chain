use serde::{Serialize, Deserialize};

extern crate diesel;

use crate::schema::users;
use diesel::sql_types::Integer;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub eth_key: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}