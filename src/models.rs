use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub eth_key: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}