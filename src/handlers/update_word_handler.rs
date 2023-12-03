use std::sync::Arc;

use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::associations::HasTable;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde_json::json;
use uuid::Uuid;

use crate::db_util::execute_in_db;
use crate::dtos::UpdateWordDto;
use crate::extractors::UserIdentifier;
use crate::schema::words::dsl::words;
use crate::schema::words::{id as word_table_id, source, translation, word as word_column};
use crate::validators::validate_user_access;
use crate::AppState;

pub async fn update_word(
    Path(word_id): Path<Uuid>,
    UserIdentifier(user_id): UserIdentifier,
    State(db): State<Arc<AppState>>,
    Json(body): Json<UpdateWordDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validate_user_access(db.clone(), &user_id, &word_id)
        .await
        .map_err(|status| (status, Json(json!({ "error": "Word not found" }))))?;

    execute_in_db(db.clone(), move |conn| {
        diesel::update(words::table().filter(word_table_id.eq(word_id)))
            .set((
                word_column.eq(body.word),
                translation.eq(body.translation),
                source.eq(body.source),
            ))
            .execute(conn)
            .expect("Error updating word")
    })
    .await;

    let json_response = json!({
        "status": "success",
    });

    Ok(Json(json_response))
}
