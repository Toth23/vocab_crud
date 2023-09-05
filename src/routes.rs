use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState,
    handlers::list_vocab_handler,
};
use crate::handlers::{create_word_handler, health_checker_handler};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/vocab", get(list_vocab_handler))
        .route("/api/vocab", post(create_word_handler))
        .with_state(app_state)
}