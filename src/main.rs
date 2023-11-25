use std::env;
use std::net::SocketAddr;

use dotenvy::dotenv;
use vocab_crud::create_app;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in env");
    let app = create_app(database_url);

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_owned())
        .parse::<u16>()
        .expect("Port not valid");
    let addr: SocketAddr = format!("{host}:{port}")
        .parse()
        .expect("Not a valid socket address");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
