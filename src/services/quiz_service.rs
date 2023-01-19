use crate::db::establish_connection;
use crate::models::quiz::{NewQuiz, Quiz};
use crate::schema::quizs;
use diesel::prelude::*;

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

pub fn get_quiz(id: i32) -> Quiz {
    let connection = &mut establish_connection();
    quizs::table.find(id).first(connection).unwrap()
}
