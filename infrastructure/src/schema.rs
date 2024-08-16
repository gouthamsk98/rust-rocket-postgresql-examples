// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        genre -> Varchar,
        published -> Bool,
    }
}

diesel::table! {
    test (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        genre -> Varchar,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    test,
);
