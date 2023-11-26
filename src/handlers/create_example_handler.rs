use std::sync::Arc;

use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::associations::HasTable;
use diesel::{RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::db_util::execute_in_db;
use crate::dtos::CreateExampleDto;
use crate::extractors::UserIdentifier;
use crate::mappers::map_example_to_response;
use crate::models::{Example, NewExample};
use crate::schema::examples::dsl::examples;
use crate::AppState;

pub async fn create_example(
    Path(word_id): Path<Uuid>,
    UserIdentifier(_user_id): UserIdentifier,
    State(db): State<Arc<AppState>>,
    Json(body): Json<CreateExampleDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let app_state: Arc<AppState> = db.clone();

    let new_example = NewExample {
        word_id,
        example: body.example,
    };

    let example = execute_in_db(app_state, move |conn| {
        diesel::insert_into(examples::table())
            .values(&new_example)
            .returning(Example::as_returning())
            .get_result(conn)
            .expect("Error saving new example")
    })
    .await;

    let example_response = map_example_to_response(&example);
    let json_response = serde_json::json!({
        "status": "success",
        "example": example_response,
    });

    Ok(Json(json_response))
}
