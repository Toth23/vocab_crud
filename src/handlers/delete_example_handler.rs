use std::sync::Arc;

use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::associations::HasTable;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde_json::json;
use uuid::Uuid;

use crate::db_util::{execute_in_db, validate_user_access};
use crate::schema::examples::dsl::examples;
use crate::schema::examples::id as example_table_id;
use crate::schema::examples::word_id as example_table_word_id;
use crate::AppState;
use crate::extractors::UserIdentifier;

pub async fn delete_example(
    Path((word_id, example_id)): Path<(Uuid, Uuid)>,
    UserIdentifier(user_id): UserIdentifier,
    State(db): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validate_user_access(db.clone(), &user_id, &word_id)
        .await
        .map_err(|status| (status, Json(json!({ "error": "Word not found" }))))?;

    let number_deleted = execute_in_db(db.clone(), move |conn| {
        diesel::delete(
            examples::table().filter(
                example_table_id
                    .eq(example_id)
                    .and(example_table_word_id.eq(word_id)),
            ),
        )
        .execute(conn)
        .expect("Error deleting word")
    })
    .await;

    let json_response = json!({
        "status": "success",
        "number_deleted": number_deleted,
    });

    Ok(Json(json_response))
}
