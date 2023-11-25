use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::handlers::create_example_handler::create_example;
use crate::handlers::create_word_handler::create_word;
use crate::handlers::delete_example_handler::delete_example;
use crate::handlers::delete_word_handler::delete_word;
use crate::handlers::health_check_handler::check_health;
use crate::handlers::list_handler::list_vocab;
use crate::handlers::update_word_handler::update_word;
use crate::AppState;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health", get(check_health))
        .route("/api/vocab", get(list_vocab))
        .route("/api/vocab", post(create_word))
        .route("/api/vocab/:word_id", put(update_word))
        .route("/api/vocab/:word_id", delete(delete_word))
        .route("/api/vocab/:word_id/examples", post(create_example))
        .route(
            "/api/vocab/:word_id/examples/:example_id",
            delete(delete_example),
        )
        .with_state(app_state)
}
