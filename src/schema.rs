// @generated automatically by Diesel CLI.

diesel::table! {
    records (id) {
        id -> Varchar,
        url -> Text,
        password -> Text,
        expired_at -> Timestamp,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}