use crate::schema::users;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};
use serde_json;

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub score: Option<i32>,
    pub created: Option<std::time::SystemTime>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: Option<String>,
    pub score: Option<i32>,
    pub created: Option<std::time::SystemTime>,
}
