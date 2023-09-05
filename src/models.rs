use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::words)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub translation: Option<String>,
    pub date_added: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::words)]
pub struct NewWord {
    pub word: String,
    pub translation: Option<String>,
    pub date_added: String,
}
