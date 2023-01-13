// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Nullable<Text>,
        score -> Nullable<Int4>,
        created -> Nullable<Timestamp>,
    }
}
