use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::words)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub translation: Option<String>,
    pub source: Option<String>,
    pub date_added: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::words)]
pub struct NewWord {
    pub word: String,
    pub translation: Option<String>,
    pub source: Option<String>,
    pub date_added: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Word))]
#[diesel(table_name = crate::schema::examples)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Example {
    pub id: i32,
    pub word_id: i32,
    pub example: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::examples)]
pub struct NewExample {
    pub word_id: i32,
    pub example: String,
}
