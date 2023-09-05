use std::sync::Arc;

use axum::{
    routing::{get},
    Router,
};

use crate::{
    handlers::{
        list_vocab_handler
    },
    AppState,
};
use crate::handlers::health_checker_handler;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/vocab/", get(list_vocab_handler))
        .with_state(app_state)
}