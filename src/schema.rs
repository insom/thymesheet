// @generated automatically by Diesel CLI.

diesel::table! {
    days (id) {
        id -> Integer,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    weeks (id) {
        id -> Integer,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    days,
    weeks,
);
