// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        content -> Text,
        posted_at -> Timestamp,
    }
}
