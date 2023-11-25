use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use diesel::{BelongingToDsl, GroupedBy, QueryDsl, RunQueryDsl, SelectableHelper};
use serde::Deserialize;

use crate::db_util::execute_in_db;
use crate::dtos::VocabResponseDto;
use crate::mappers::map_word_to_response;
use crate::models::Example;
use crate::schema::words::dsl::words;
use crate::{models::Word, AppState};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

pub async fn list_vocab(
    opts: Option<Query<FilterOptions>>,
    State(db): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(1000);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let app_state: Arc<AppState> = db.clone();

    let (db_words, db_examples) = execute_in_db(app_state, move |conn| {
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
    })
    .await;

    let words_with_examples = db_examples
        .grouped_by(&db_words)
        .into_iter()
        .zip(db_words)
        .collect::<Vec<(Vec<Example>, Word)>>();

    let items = words_with_examples
        .iter()
        .map(|(word_examples, word)| map_word_to_response(word, word_examples))
        .collect::<Vec<VocabResponseDto>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results": items.len(),
        "words": items
    });

    Ok(Json(json_response))
}
