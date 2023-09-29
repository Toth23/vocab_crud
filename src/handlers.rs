use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use axum::extract::Path;
use chrono;
use diesel::{BelongingToDsl, BoolExpressionMethods, ExpressionMethods, GroupedBy, QueryDsl, RunQueryDsl, SelectableHelper, sql_query};
use diesel::associations::HasTable;
use serde::Deserialize;

use crate::{
    AppState,
    models::Word,
};
use crate::dtos::{CreateExampleDto, CreateWordDto, ExampleResponseDto, UpdateWordDto, VocabResponseDto};
use crate::models::{Example, NewExample, NewWord};
use crate::schema::examples::dsl::examples;
use crate::schema::examples::id as example_table_id;
use crate::schema::examples::word_id as example_table_word_id;
use crate::schema::words::{id as word_table_id, source, translation, word as word_column};
use crate::schema::words::dsl::words;

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}


pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "It's running!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn list_vocab_handler(
    opts: Option<Query<FilterOptions>>,
    State(db): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(1000);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let app_state: Arc<AppState> = db.clone();
    let (db_words, db_examples) = app_state.db.get().await.expect("Failed to get database connection")
        .interact(move |conn| {
            let db_words = words
                .limit(limit as i64)
                .offset(offset as i64)
                .select(Word::as_select())
                .load(conn)
                .expect("Error loading words");
            let db_examples = Example::belonging_to(&db_words)
                .select(Example::as_select())
                .load(conn)
                .expect("Error loading examples");
            (db_words, db_examples)
        }).await.expect("Error interacting with the database");

    let words_with_examples = db_examples
        .grouped_by(&db_words)
        .into_iter()
        .zip(db_words)
        .collect::<Vec<(Vec<Example>, Word)>>();

    let items = words_with_examples.iter()
        .map(|(word_examples, word)| map_word_to_response(word, word_examples))
        .collect::<Vec<VocabResponseDto>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results": items.len(),
        "words": items
    });

    Ok(Json(json_response))
}

pub async fn create_word_handler(
    State(db): State<Arc<AppState>>,
    Json(body): Json<CreateWordDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let app_state: Arc<AppState> = db.clone();

    let date_time_now = chrono::offset::Utc::now();

    let new_word = NewWord {
        word: body.word,
        translation: body.translation,
        source: body.source,
        date_added: date_time_now.format("%d.%m.%Y").to_string(),
    };

    let word = app_state.db.get().await.expect("Failed to get database connection")
        .interact(move |conn| {
            diesel::insert_into(words::table())
                .values(&new_word)
                .returning(Word::as_returning())
                .get_result(conn)
                .expect("Error saving new word")
        }).await.expect("Error interacting with the database");

    let json_response = serde_json::json!({
        "status": "success",
        "word": map_word_to_response(&word, &vec![]),
    });

    Ok(Json(json_response))
}

pub async fn update_word_handler(
    Path(word_id): Path<i32>,
    State(db): State<Arc<AppState>>,
    Json(body): Json<UpdateWordDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let app_state: Arc<AppState> = db.clone();

    app_state.db.get().await.expect("Failed to get database connection")
        .interact(move |conn| {
            diesel::update(words::table().filter(word_table_id.eq(word_id)))
                .set((
                    word_column.eq(body.word),
                    translation.eq(body.translation),
                    source.eq(body.source),
                ))
                .execute(conn)
                .expect("Error updating word")
        }).await.expect("Error interacting with the database");

    let json_response = serde_json::json!({
        "status": "success",
    });

    Ok(Json(json_response))
}

pub async fn delete_word_handler(
    Path(word_id): Path<i32>,
    State(db): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let app_state: Arc<AppState> = db.clone();

    app_state.db.get().await.expect("Failed to get database connection")
        .interact(move |conn| {
            diesel::delete(examples::table()
                .filter(example_table_word_id.eq(word_id)))
                .execute(conn)
                .expect("Error deleting examples of word");

            diesel::delete(words::table().filter(word_table_id.eq(word_id)))
                .execute(conn)
                .expect("Error deleting word")
        }).await.expect("Error interacting with the database");

    let json_response = serde_json::json!({
        "status": "success",
    });

    Ok(Json(json_response))
}

pub async fn create_example_handler(
    Path(word_id): Path<i32>,
    State(db): State<Arc<AppState>>,
    Json(body): Json<CreateExampleDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let app_state: Arc<AppState> = db.clone();

    let new_example = NewExample { word_id, example: body.example };

    let example = app_state.db.get().await.expect("Failed to get database connection")
        .interact(move |conn| {
            sql_query("PRAGMA foreign_keys = ON").execute(conn).expect("Error enabling foreign keys");

            diesel::insert_into(examples::table())
                .values(&new_example)
                .returning(Example::as_returning())
                .get_result(conn)
                .expect("Error saving new example")
        }).await.expect("Error interacting with the database");

    let json_response = serde_json::json!({
        "status": "success",
        "example": map_example_to_response(&example)
    });

    Ok(Json(json_response))
}


pub async fn delete_example_handler(
    Path((word_id, example_id)): Path<(i32, i32)>,
    State(db): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let app_state: Arc<AppState> = db.clone();

    let number_deleted = app_state.db.get().await
        .expect("Failed to get database connection")
        .interact(move |conn| {
            diesel::delete(examples::table()
                .filter(example_table_id.eq(example_id)
                    .and(example_table_word_id.eq(word_id))))
                .execute(conn)
                .expect("Error deleting word")
        }).await.expect("Error interacting with the database");

    let json_response = serde_json::json!({
        "status": "success",
        "number_deleted": number_deleted,
    });

    Ok(Json(json_response))
}

fn map_word_to_response(word: &Word, word_examples: &Vec<Example>) -> VocabResponseDto {
    VocabResponseDto {
        id: word.id,
        word: word.word.to_owned(),
        translation: word.translation.to_owned(),
        source: word.source.to_owned(),
        examples: word_examples.into_iter().map(map_example_to_response).collect(),
        date_added: word.date_added.to_owned(),
    }
}

fn map_example_to_response(example: &Example) -> ExampleResponseDto {
    ExampleResponseDto { id: example.id, example: example.example.to_owned() }
}