table! {
    post (id) {
        id -> Integer,
        body -> Text,
        user_id -> Integer,
    }
}

table! {
    user (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    post,
    user,
);
