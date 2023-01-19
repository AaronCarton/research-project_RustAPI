use crate::models::quiz::{NewQuiz, Quiz};
use crate::services::quiz_service;
use rocket::serde::json::Json;

#[post("/", data = "<payload>")]
pub fn create_quiz(payload: Json<NewQuiz>) -> Json<Quiz> {
    let quiz = payload.into_inner();
    Json(quiz_service::create_quiz(quiz))
}

#[get("/")]
pub fn get_quizs() -> Json<Vec<Quiz>> {
    Json(quiz_service::get_quizs())
}

#[get("/<id>")]
pub fn get_quiz(id: i32) -> Json<Quiz> {
    Json(quiz_service::get_quiz(id))
}
