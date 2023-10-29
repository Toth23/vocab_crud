extern crate diesel_migrations;

use std::collections::HashMap;

use diesel::Connection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use serde_json::Value;
use serde_json::Value::Array;

use vocab_crud::create_app;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::test]
async fn health_is_ok() {
    let port = spawn_test_server();

    let client = reqwest::Client::new();
    let resp = client.get(format!("http://localhost:{port}/api/health"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn gives_an_empty_list() {
    let port = spawn_test_server();

    let client = reqwest::Client::new();
    let resp = client.get(format!("http://localhost:{port}/api/vocab"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let body: HashMap<String, Value> = resp.json().await.unwrap();
    assert_eq!(body["words"], Array(vec![]));
}

fn spawn_test_server() -> u16 {
    let database_url = "integration-test.db".to_owned();
    let mut connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    connection.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");

    let app = create_app(database_url);

    let server = axum::Server::bind(&"0.0.0.0:0".parse().unwrap())
        .serve(app.into_make_service());
    let port = server.local_addr().port();

    tokio::spawn(server);

    port
}