// @generated automatically by Diesel CLI.

diesel::table! {
    question (id) {
        id -> Integer,
        question -> Varchar,
        answers -> Text,
        correct_answer -> Varchar,
    }
}

diesel::table! {
    quiz (id) {
        id -> Integer,
        name -> Varchar,
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
    user (id) {
        id -> Integer,
        uid -> Varchar,
        role -> Integer,
        username -> Varchar,
        score -> Integer,
    }
}

diesel::table! {
    user_history (id) {
        id -> Integer,
        user_id -> Integer,
        question_id -> Integer,
        answer -> Varchar,
    }
}

diesel::joinable!(quiz_questions -> question (question_id));
diesel::joinable!(quiz_questions -> quiz (quiz_id));
diesel::joinable!(user_history -> question (question_id));
diesel::joinable!(user_history -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    question,
    quiz,
    quiz_questions,
    user,
    user_history,
);
