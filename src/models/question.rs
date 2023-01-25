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

#[derive(Queryable, Serialize, Deserialize)]
pub struct QuestionModel {
    pub id: i32,
    pub question: String,
    pub answers: Vec<Answer>,
}

// answer model
#[derive(Queryable, Serialize, Deserialize)]
pub struct Answer {
    pub answer: String,
    pub is_correct: bool,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct AnswerQuestionModel {
    pub answer: String,
}
