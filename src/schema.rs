// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        uid -> Varchar,
        role -> Integer,
        username -> Varchar,
        score -> Integer,
    }
}
