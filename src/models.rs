use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::words)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub translation: Option<String>,
    pub date_added: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VocabResponse {
    pub id: i32,
    pub word: String,
    pub translation: Option<String>,
    pub date_added: String,
}