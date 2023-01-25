use crate::db::establish_connection;
use crate::models::question::{Answer, QuestionModel};
use crate::models::quiz::{NewQuiz, Quiz, QuizModel};
use crate::schema::quizs;
use diesel::prelude::*;

use super::question_service;

pub fn create_quiz(new: NewQuiz) -> Quiz {
    use crate::schema::quizs::dsl::*;

    let connection = &mut establish_connection();
    diesel::insert_into(quizs)
        .values(&new)
        .execute(connection)
        .expect("Error saving new Quiz");

    quizs.order(id.desc()).first(connection).unwrap()
}

pub fn get_quizs() -> Vec<Quiz> {
    use crate::schema::quizs::dsl::*;
    let connection = &mut establish_connection();
    let results = quizs.load::<Quiz>(connection).unwrap();

    results
}

pub fn get_quiz(id: i32) -> QuizModel {
    let connection = &mut establish_connection();
    let quiz: Quiz = quizs::table.find(id).first::<Quiz>(connection).unwrap();
    let questions = question_service::get_questions_by_quiz(id);

    // order questions by id
    let mut questions = questions;
    questions.sort_by(|a, b| a.id.cmp(&b.id));

    // for each quesiton, convert answers to a vector of Answer
    let questions: Vec<QuestionModel> = questions
        .into_iter()
        .map(|q| {
            // convert answers to a vector of Answer models
            let answers = q
                .answers
                .split(',')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|a| Answer {
                    answer: a.to_string(),
                    is_correct: a == q.correct_answer,
                })
                .collect();
            // create custom question model
            QuestionModel {
                id: q.id,
                question: q.question,
                answers,
            }
        })
        .collect();

    QuizModel {
        id: quiz.id,
        name: quiz.name,
        description: quiz.description,
        questions,
    }
}
