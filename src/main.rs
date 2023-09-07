use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use deadpool_diesel::sqlite::{Manager, Pool, Runtime};
use dotenvy::dotenv;
use tower_http::cors::{Any, CorsLayer};

use routes::create_router;

mod handlers;
mod models;
mod routes;
mod schema;
mod dtos;

pub struct AppState {
    db: Pool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = Manager::new(database_url, Runtime::Tokio1);
    let pool = Pool::builder(manager)
        .max_size(8)
        .build()
        .unwrap();

    let app = create_router(Arc::new(AppState { db: pool }))
        .layer(CorsLayer::new().allow_methods(Any).allow_headers(Any).allow_origin(Any));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
