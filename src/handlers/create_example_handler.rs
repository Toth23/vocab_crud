use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use axum::extract::Path;
use diesel::{RunQueryDsl, SelectableHelper, sql_query};
use diesel::associations::HasTable;

use crate::AppState;
use crate::db_util::execute_in_db;
use crate::dtos::CreateExampleDto;
use crate::mappers::map_example_to_response;
use crate::models::{Example, NewExample};
use crate::schema::examples::dsl::examples;

pub async fn create_example(
    Path(word_id): Path<i32>,
    State(db): State<Arc<AppState>>,
    Json(body): Json<CreateExampleDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let app_state: Arc<AppState> = db.clone();

    let new_example = NewExample { word_id, example: body.example };

    let example = execute_in_db(app_state, move |conn| {
        sql_query("PRAGMA foreign_keys = ON").execute(conn).expect("Error enabling foreign keys");

        diesel::insert_into(examples::table())
            .values(&new_example)
            .returning(Example::as_returning())
            .get_result(conn)
            .expect("Error saving new example")
    }).await;

    let json_response = serde_json::json!({
        "status": "success",
        "example": map_example_to_response(&example)
    });

    Ok(Json(json_response))
}
