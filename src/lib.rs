use std::sync::Arc;

use axum::Router;
use deadpool_diesel::postgres::{Manager, Pool, Runtime};
use tower_http::cors::{Any, CorsLayer};

use routes::create_router;

mod db_util;
pub mod dtos;
mod handlers;
mod mappers;
mod models;
mod routes;
mod schema;
mod extractors;

pub struct AppState {
    db: Pool,
}

pub fn create_app(database_url: String) -> Router {
    let manager = Manager::new(database_url, Runtime::Tokio1);
    let pool = Pool::builder(manager).max_size(8).build().unwrap();

    create_router(Arc::new(AppState { db: pool })).layer(
        CorsLayer::new()
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_origin(Any),
    )
}
