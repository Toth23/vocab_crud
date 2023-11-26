use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::words)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Word {
    pub id: Uuid,
    pub word: String,
    pub translation: Option<String>,
    pub source: Option<String>,
    pub date_added: NaiveDateTime,
    pub user_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::words)]
pub struct NewWord {
    pub word: String,
    pub translation: Option<String>,
    pub source: Option<String>,
    pub date_added: NaiveDateTime,
    pub user_id: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Word))]
#[diesel(table_name = crate::schema::examples)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Example {
    pub id: Uuid,
    pub word_id: Uuid,
    pub example: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::examples)]
pub struct NewExample {
    pub word_id: Uuid,
    pub example: String,
}
