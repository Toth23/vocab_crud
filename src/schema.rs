// @generated automatically by Diesel CLI.

diesel::table! {
    examples (id) {
        id -> Uuid,
        word_id -> Uuid,
        example -> Text,
    }
}

diesel::table! {
    words (id) {
        id -> Uuid,
        word -> Text,
        translation -> Nullable<Text>,
        date_added -> Timestamp,
        source -> Nullable<Text>,
    }
}

diesel::joinable!(examples -> words (word_id));

diesel::allow_tables_to_appear_in_same_query!(
    examples,
    words,
);
