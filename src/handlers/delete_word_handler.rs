use std::sync::Arc;

use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::associations::HasTable;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde_json::json;
use uuid::Uuid;

use crate::db_util::execute_in_db;
use crate::extractors::UserIdentifier;
use crate::schema::examples::dsl::examples;
use crate::schema::examples::word_id as example_table_word_id;
use crate::schema::words::dsl::words;
use crate::schema::words::id as word_table_id;
use crate::validators::validate_user_access;
use crate::AppState;

pub async fn delete_word(
    Path(word_id): Path<Uuid>,
    UserIdentifier(user_id): UserIdentifier,
    State(db): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validate_user_access(db.clone(), &user_id, &word_id)
        .await
        .map_err(|status| (status, Json(json!({ "error": "Word not found" }))))?;

    execute_in_db(db.clone(), move |conn| {
        diesel::delete(examples::table().filter(example_table_word_id.eq(word_id)))
            .execute(conn)
            .expect("Error deleting examples of word");

        diesel::delete(words::table().filter(word_table_id.eq(word_id)))
            .execute(conn)
            .expect("Error deleting word")
    })
    .await;

    let json_response = json!({
        "status": "success",
    });

    Ok(Json(json_response))
}
