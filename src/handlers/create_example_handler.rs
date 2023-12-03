use std::sync::Arc;

use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::associations::HasTable;
use diesel::{RunQueryDsl, SelectableHelper};
use serde_json::json;
use uuid::Uuid;

use crate::db_util::execute_in_db;
use crate::dtos::CreateExampleDto;
use crate::extractors::UserIdentifier;
use crate::mappers::map_example_to_response;
use crate::models::{Example, NewExample};
use crate::schema::examples::dsl::examples;
use crate::validators::validate_user_access;
use crate::AppState;

pub async fn create_example(
    Path(word_id): Path<Uuid>,
    UserIdentifier(user_id): UserIdentifier,
    State(db): State<Arc<AppState>>,
    Json(body): Json<CreateExampleDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validate_user_access(db.clone(), &user_id, &word_id)
        .await
        .map_err(|status| (status, Json(json!({ "error": "Word not found" }))))?;

    let new_example = NewExample {
        word_id,
        example: body.example,
    };

    let example = execute_in_db(db.clone(), move |conn| {
        diesel::insert_into(examples::table())
            .values(&new_example)
            .returning(Example::as_returning())
            .get_result(conn)
            .expect("Error saving new example")
    })
    .await;

    let example_response = map_example_to_response(&example);
    let json_response = json!({
        "status": "success",
        "example": example_response,
    });

    Ok(Json(json_response))
}
