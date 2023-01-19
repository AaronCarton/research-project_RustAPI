use crate::schema::users;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub uid: String,
    pub role: i32,
    pub username: String,
    pub score: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub uid: String,
    pub role: Option<i32>,
    pub username: Option<String>,
    pub score: Option<i32>,
}
