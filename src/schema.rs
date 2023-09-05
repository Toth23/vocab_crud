// @generated automatically by Diesel CLI.

diesel::table! {
    examples (id) {
        id -> Integer,
        word_id -> Integer,
        example -> Text,
    }
}

diesel::table! {
    words (id) {
        id -> Integer,
        word -> Text,
        translation -> Nullable<Text>,
        date_added -> Text,
        source -> Nullable<Text>,
    }
}

diesel::joinable!(examples -> words (word_id));

diesel::allow_tables_to_appear_in_same_query!(
    examples,
    words,
);
