// @generated automatically by Diesel CLI.

diesel::table! {
    words (id) {
        id -> Integer,
        word -> Text,
        translation -> Nullable<Text>,
        date_added -> Text,
        source -> Nullable<Text>,
    }
}
