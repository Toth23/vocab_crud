use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use axum::extract::Path;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::associations::HasTable;
use uuid::Uuid;

use crate::AppState;
use crate::db_util::execute_in_db;
use crate::schema::examples::dsl::examples;
use crate::schema::examples::word_id as example_table_word_id;
use crate::schema::words::dsl::words;
use crate::schema::words::id as word_table_id;

pub async fn delete_word(
    Path(word_id): Path<Uuid>,
    State(db): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let app_state: Arc<AppState> = db.clone();

    execute_in_db(app_state, move |conn| {
        diesel::delete(examples::table()
            .filter(example_table_word_id.eq(word_id)))
            .execute(conn)
            .expect("Error deleting examples of word");

        diesel::delete(words::table().filter(word_table_id.eq(word_id)))
            .execute(conn)
            .expect("Error deleting word")
    }).await;

    let json_response = serde_json::json!({
        "status": "success",
    });

    Ok(Json(json_response))
}