use std::sync::Arc;

use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::associations::HasTable;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::db_util::execute_in_db;
use crate::schema::examples::dsl::examples;
use crate::schema::examples::word_id as example_table_word_id;
use crate::schema::words::dsl::words;
use crate::schema::words::id as word_table_id;
use crate::AppState;
use crate::extractors::UserIdentifier;

pub async fn delete_word(
    Path(word_id): Path<Uuid>,
    UserIdentifier(_user_id): UserIdentifier,
    State(db): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let app_state: Arc<AppState> = db.clone();

    execute_in_db(app_state, move |conn| {
        diesel::delete(examples::table().filter(example_table_word_id.eq(word_id)))
            .execute(conn)
            .expect("Error deleting examples of word");

        diesel::delete(words::table().filter(word_table_id.eq(word_id)))
            .execute(conn)
            .expect("Error deleting word")
    })
    .await;

    let json_response = serde_json::json!({
        "status": "success",
    });

    Ok(Json(json_response))
}
