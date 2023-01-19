use crate::schema::quizs;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = quizs)]
pub struct Quiz {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = quizs)]
pub struct NewQuiz {
    pub name: String,
}
