use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use diesel::associations::HasTable;
use diesel::{RunQueryDsl, SelectableHelper};

use crate::db_util::execute_in_db;
use crate::dtos::CreateWordDto;
use crate::extractors::UserIdentifier;
use crate::mappers::map_word_to_response;
use crate::models::Word;
use crate::models::{Example, NewExample, NewWord};
use crate::schema::examples::dsl::examples;
use crate::schema::words::dsl::words;
use crate::AppState;

pub async fn create_word(
    UserIdentifier(user_id): UserIdentifier,
    State(db): State<Arc<AppState>>,
    Json(body): Json<CreateWordDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let app_state: Arc<AppState> = db.clone();

    let date_time_now = Utc::now().naive_utc();

    let new_word = NewWord {
        word: body.word,
        translation: body.translation,
        source: body.source,
        date_added: date_time_now,
        user_id
    };

    let (word, word_examples) = execute_in_db(app_state, move |conn| {
        let word = diesel::insert_into(words::table())
            .values(&new_word)
            .returning(Word::as_returning())
            .get_result(conn)
            .expect("Error saving new word");

        let word_examples: Vec<Example> = body
            .examples
            .iter()
            .map(|ex| {
                diesel::insert_into(examples::table())
                    .values(NewExample {
                        word_id: word.id,
                        example: ex.to_owned(),
                    })
                    .returning(Example::as_returning())
                    .get_result(conn)
                    .expect("Error saving new example")
            })
            .collect();

        (word, word_examples)
    })
    .await;

    let word_response = map_word_to_response(&word, &word_examples);
    let json_response = serde_json::json!({
        "status": "success",
        "word": word_response,
    });

    Ok(Json(json_response))
}
