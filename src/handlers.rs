use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use serde::Deserialize;

use crate::{
    AppState,
    models::Word,
};
use crate::models::VocabResponse;
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
        .collect::<Vec<VocabResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results": items.len(),
        "notes": items
    });

    Ok(Json(json_response))
}

fn map_to_response(record: &Word) -> VocabResponse {
    VocabResponse {
        id: record.id.to_owned(),
        word: record.word.to_owned(),
        translation: record.translation.to_owned(),
        date_added: record.date_added.to_owned(),
    }
}