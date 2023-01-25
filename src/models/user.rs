use crate::schema::user_history;
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
    pub quiz_id: i32,
    pub question_id: i32,
    pub question: String,
    pub answer: String,
    pub correct: bool,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserModel {
    pub id: i32,
    pub uid: String,
    pub role: i32,
    pub username: String,
    pub score: i32,
    pub history: Vec<UserHistory>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub uid: Option<String>,
    pub username: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = user_history)]
pub struct NewUserHistory {
    pub user_id: i32,
    pub quiz_id: i32,
    pub question_id: i32,
    pub answer: String,
    pub correct: bool,
}
