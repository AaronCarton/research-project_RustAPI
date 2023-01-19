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

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = user_history)]
pub struct UserHistory {
    pub question_id: i32,
    pub question: String,
    pub answer: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserModel {
    pub uid: String,
    pub role: i32,
    pub username: String,
    pub score: i32,
    pub history: Vec<UserHistory>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub uid: String,
    pub role: Option<i32>,
    pub username: Option<String>,
    pub score: Option<i32>,
}
