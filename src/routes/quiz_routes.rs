use crate::guards::adminguard::AdminGuard;
use crate::guards::authguard::AuthGuard;
use crate::models::clientresponse::ClientResponse;
use crate::models::question::AnswerQuestionModel;
use crate::models::quiz::{NewQuiz, Quiz, QuizModel};
use crate::services::{quiz_service, user_service};
use rocket::serde::json::Json;

#[post("/", data = "<payload>")]
pub fn create_quiz(_admin: AdminGuard, payload: Json<NewQuiz>) -> Json<Quiz> {
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

#[post("/<quiz_id>/answer/<question_id>", data = "<answer>")]
pub fn answer_quiz_question(
    auth: AuthGuard,
    quiz_id: i32,
    question_id: i32,
    answer: Json<AnswerQuestionModel>,
) -> Json<ClientResponse> {
    Json(user_service::answer_question(
        auth.uid,
        quiz_id,
        question_id,
        answer.into_inner().answer,
    ))
}

#[get("/<quiz_id>/score")]
pub fn get_quiz_score(auth: AuthGuard, quiz_id: i32) -> Json<i32> {
    Json(user_service::get_quiz_score(auth.uid, quiz_id))
}
