use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = questions)]
pub struct Question {
    pub id: i32,
    pub question: String,
    pub answers: String,
    pub correct_answer: String,
}
