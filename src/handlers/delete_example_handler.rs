use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use axum::extract::Path;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::associations::HasTable;

use crate::AppState;
use crate::db_util::execute_in_db;
use crate::schema::examples::dsl::examples;
use crate::schema::examples::id as example_table_id;
use crate::schema::examples::word_id as example_table_word_id;

pub async fn delete_example(
    Path((word_id, example_id)): Path<(i32, i32)>,
    State(db): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let app_state: Arc<AppState> = db.clone();

    let number_deleted = execute_in_db(app_state, move |conn| {
        diesel::delete(examples::table()
            .filter(example_table_id.eq(example_id)
                .and(example_table_word_id.eq(word_id))))
            .execute(conn)
            .expect("Error deleting word")
    }).await;

    let json_response = serde_json::json!({
        "status": "success",
        "number_deleted": number_deleted,
    });

    Ok(Json(json_response))
}
