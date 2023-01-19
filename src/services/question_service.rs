use crate::db::establish_connection;
use crate::models::question::Question;
use crate::schema::questions;
use crate::schema::quiz_questions;
use diesel::prelude::*;

pub fn get_questions_by_quiz(quiz_id: i32) -> Vec<Question> {
    let connection = &mut establish_connection();
    let results = quiz_questions::table
        .filter(quiz_questions::quiz_id.eq(quiz_id))
        .inner_join(questions::table)
        .select(questions::all_columns)
        .load::<Question>(connection)
        .expect("Error loading questions");

    results
}
