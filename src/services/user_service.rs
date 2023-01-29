use crate::db::establish_connection;
use crate::models::clientresponse::ClientResponse;
use crate::models::question::Question;
use crate::models::user::{NewUser, NewUserHistory, User, UserHistory, UserModel};
use crate::schema::{questions, quizs, user_history, users};
use diesel::prelude::*;

pub fn create_user(new_user: NewUser) -> User {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    diesel::insert_into(users)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new user");

    users.order(id.desc()).first(connection).unwrap()
}

pub fn get_users() -> Vec<User> {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    let results = users.load::<User>(connection).unwrap();

    results
}

pub fn get_user(id: i32) -> UserModel {
    let connection = &mut establish_connection();
    let user = users::table.find(id).first(connection).unwrap();

    join_user(user)
}

pub fn get_quiz_score(uid: String, quiz_id: i32) -> i32 {
    let user = get_user_by_uid(uid);

    let score = user
        .history
        .iter()
        .filter(|h| h.quiz_id == quiz_id)
        .map(|h| h.correct)
        .filter(|c| *c)
        .count() as i32;

    score
}

pub fn answer_question(
    uid: String,
    quiz_id: i32,
    question_id: i32,
    answer: String,
) -> ClientResponse {
    let connection = &mut establish_connection();
    let user = get_user_by_uid(uid.clone());
    let question: Question = questions::table
        .find(question_id)
        .first(connection)
        .unwrap();

    // check if user has already answered this question and return error if so
    let res = user
        .history
        .iter()
        .map(|h| h.question_id)
        .find(|&q| q == question_id);
    if res.is_some() {
        return ClientResponse {
            success: false,
            status: 400,
            message: String::from("User has already answered this question"),
            data: serde_json::json!({}),
        };
    }

    // check if answer is correct and update score if so
    let correct = question.correct_answer == answer;
    if correct {
        increase_score(user.id);
    }

    // add user history row
    add_user_history_row(user.id, quiz_id, question_id, answer, correct);

    return ClientResponse {
        success: true,
        status: 200,
        message: String::from(""),
        data: serde_json::json!({
            "correct": correct,
            "correct_answer": question.correct_answer,
            "total_score": get_user(user.id).score,
        }),
    };
}

pub fn add_user_history_row(
    user_id: i32,
    quiz_id: i32,
    question_id: i32,
    answer: String,
    correct: bool,
) {
    let connection = &mut establish_connection();

    // create new user history row
    let new_user_history = NewUserHistory {
        user_id,
        quiz_id,
        question_id,
        answer,
        correct,
    };
    // insert
    diesel::insert_into(user_history::table)
        .values(&new_user_history)
        .execute(connection)
        .expect("Error saving new user history");
}

pub fn get_user_by_uid(uid: String) -> UserModel {
    let connection = &mut establish_connection();
    let user = users::table
        .filter(users::columns::uid.eq(uid))
        .first(connection)
        .unwrap();

    join_user(user)
}

// pub fn delete_user(id: i32) {
//     let connection = &mut establish_connection();
//     diesel::delete(users::table.find(id))
//         .execute(connection)
//         .unwrap();
// }

pub fn reset_user(uid: String) -> UserModel {
    let connection = &mut establish_connection();
    let user = get_user_by_uid(uid.clone());
    let new_score = 0;

    // reset score
    diesel::update(users::table.find(user.id))
        .set(users::score.eq(new_score))
        .execute(connection)
        .unwrap();

    // delete history
    diesel::delete(user_history::table.filter(user_history::columns::user_id.eq(user.id)))
        .execute(connection)
        .unwrap();

    get_user_by_uid(uid)
}

pub fn increase_score(id: i32) -> UserModel {
    let connection = &mut establish_connection();
    let user = get_user(id);
    let new_score = user.score + 1;
    diesel::update(users::table.find(id))
        .set(users::score.eq(new_score))
        .execute(connection)
        .unwrap();

    get_user(id)
}

pub fn join_user(user: User) -> UserModel {
    let connection = &mut establish_connection();

    // get user's history (History table joined with Question table)
    let history = user_history::table
        .inner_join(questions::table)
        .inner_join(quizs::table)
        .filter(user_history::columns::user_id.eq(user.id))
        .select((
            quizs::columns::id,
            questions::columns::id,
            questions::columns::question,
            user_history::columns::answer,
            user_history::columns::correct,
        ))
        .load::<UserHistory>(connection)
        .unwrap();

    // combine into custom model
    let user_model = UserModel {
        id: user.id,
        uid: user.uid,
        role: user.role,
        username: user.username,
        score: user.score,
        history: history,
    };

    user_model
}
