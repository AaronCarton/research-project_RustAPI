use crate::schema::quizs;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

use super::question::QuestionModel;

#[derive(Queryable, Serialize, Deserialize)]
pub struct QuizModel {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub questions: Vec<QuestionModel>,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = quizs)]
pub struct Quiz {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = quizs)]
pub struct NewQuiz {
    pub name: String,
    pub description: String,
}
