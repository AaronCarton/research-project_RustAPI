// @generated automatically by Diesel CLI.

diesel::table! {
    questions (id) {
        id -> Integer,
        question -> Varchar,
        answers -> Text,
        correct_answer -> Varchar,
    }
}

diesel::table! {
    quiz_questions (id) {
        id -> Integer,
        quiz_id -> Integer,
        question_id -> Integer,
    }
}

diesel::table! {
    quizs (id) {
        id -> Integer,
        name -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    user_history (id) {
        id -> Integer,
        user_id -> Integer,
        quiz_id -> Integer,
        question_id -> Integer,
        answer -> Varchar,
        correct -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        uid -> Varchar,
        role -> Integer,
        username -> Varchar,
        score -> Integer,
    }
}

diesel::joinable!(quiz_questions -> questions (question_id));
diesel::joinable!(quiz_questions -> quizs (quiz_id));
diesel::joinable!(user_history -> questions (question_id));
diesel::joinable!(user_history -> quizs (quiz_id));
diesel::joinable!(user_history -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    questions,
    quiz_questions,
    quizs,
    user_history,
    users,
);
