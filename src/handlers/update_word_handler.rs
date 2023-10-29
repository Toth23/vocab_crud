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

use crate::AppState;
use crate::db_util::execute_in_db;
use crate::dtos::UpdateWordDto;
use crate::schema::words::{id as word_table_id, source, translation, word as word_column};
use crate::schema::words::dsl::words;

pub async fn update_word(
    Path(word_id): Path<i32>,
    State(db): State<Arc<AppState>>,
    Json(body): Json<UpdateWordDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let app_state: Arc<AppState> = db.clone();

    execute_in_db(app_state, move |conn| {
        diesel::update(words::table().filter(word_table_id.eq(word_id)))
            .set((
                word_column.eq(body.word),
                translation.eq(body.translation),
                source.eq(body.source),
            ))
            .execute(conn)
            .expect("Error updating word")
    }).await;

    let json_response = serde_json::json!({
        "status": "success",
    });

    Ok(Json(json_response))
}