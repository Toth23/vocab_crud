use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use axum::extract::Path;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::associations::HasTable;
use serde::Deserialize;
use chrono;

use crate::{
    AppState,
    models::Word,
};
use crate::dtos::{CreateWordDto, VocabResponseDto};
use crate::models::NewWord;
use crate::schema::words::dsl::words;
use crate::schema::words::id;

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

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let app_state: Arc<AppState> = db.clone();
    let db_records = app_state.db.get().await.expect("Failed to get database connection")
        .interact(move |conn| {
            words
                .limit(limit as i64)
                .offset(offset as i64)
                .select(Word::as_select())
                .load(conn)
                .expect("Error loading words")
        }).await.expect("Error interacting with the database");

    let items = db_records.iter()
        .map(|record| map_to_response(record))
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
        date_added: date_time_now.format("%d.%m.%Y").to_string(),
    };

    app_state.db.get().await.expect("Failed to get database connection")
        .interact(move |conn| {
            diesel::insert_into(words::table())
                .values(&new_word)
                .execute(conn)
                .expect("Error saving new word")
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
            diesel::delete(words::table().filter(id.eq(word_id)))
                .execute(conn)
                .expect("Error deleting word")
        }).await.expect("Error interacting with the database");

    let json_response = serde_json::json!({
        "status": "success",
    });

    Ok(Json(json_response))
}

fn map_to_response(record: &Word) -> VocabResponseDto {
    VocabResponseDto {
        id: record.id.to_owned(),
        word: record.word.to_owned(),
        translation: record.translation.to_owned(),
        date_added: record.date_added.to_owned(),
    }
}