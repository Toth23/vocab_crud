use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use chrono;
use diesel::{RunQueryDsl, SelectableHelper};
use diesel::associations::HasTable;

use crate::AppState;
use crate::db_util::execute_in_db;
use crate::dtos::CreateWordDto;
use crate::mappers::map_word_to_response;
use crate::models::NewWord;
use crate::models::Word;
use crate::schema::words::dsl::words;

pub async fn create_word(
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

    let word = execute_in_db(app_state, move |conn| {
        diesel::insert_into(words::table())
            .values(&new_word)
            .returning(Word::as_returning())
            .get_result(conn)
            .expect("Error saving new word")
    }).await;

    let word_response = map_word_to_response(&word, &vec![]);
    let json_response = serde_json::json!({
        "status": "success",
        "word": word_response,
    });

    Ok(Json(json_response))
}