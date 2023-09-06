use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post, delete},
};

use crate::{
    AppState,
    handlers::list_vocab_handler,
};
use crate::handlers::{create_example_handler, create_word_handler, delete_example_handler, delete_word_handler, health_checker_handler};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health", get(health_checker_handler))
        .route("/api/vocab", get(list_vocab_handler))
        .route("/api/vocab", post(create_word_handler))
        .route("/api/vocab/:word_id", delete(delete_word_handler))
        .route("/api/vocab/:word_id/examples", post(create_example_handler))
        .route("/api/vocab/:word_id/examples/example_id", delete(delete_example_handler))
        .with_state(app_state)
}