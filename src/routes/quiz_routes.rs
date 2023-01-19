use crate::guards::authguard::AuthGuard;
use crate::models::clientresponse::ClientResponse;
use crate::models::quiz::{NewQuiz, Quiz, QuizModel};
use crate::services::{quiz_service, user_service};
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
pub fn get_quiz(id: i32) -> Json<QuizModel> {
    Json(quiz_service::get_quiz(id))
}

#[post("/<id>/answer/<question_id>", data = "<answer>")]
pub fn answer_quiz_question(
    auth: AuthGuard,
    id: i32,
    question_id: i32,
    answer: String,
) -> Json<ClientResponse> {
    Json(user_service::answer_question(auth.uid, question_id, answer))
}
