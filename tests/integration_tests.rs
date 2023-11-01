extern crate diesel_migrations;

use std::collections::HashMap;

use diesel::Connection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use serde_json::Value;

use vocab_crud::create_app;
use vocab_crud::dtos::{CreateWordDto, UpdateWordDto, VocabResponseDto};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::test]
async fn health_is_ok() {
    // given
    let port = spawn_test_server();
    let client = reqwest::Client::new();

    // when
    let resp = client.get(format!("http://localhost:{port}/api/health"))
        .send()
        .await
        .unwrap();

    // then
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn gives_an_empty_list() {
    // given
    let port = spawn_test_server();

    // when
    let word_list = get_word_list(port).await;

    // then
    assert_eq!(word_list.len(), 0)
}

#[tokio::test]
async fn create_modifies_and_deletes_a_word() {
    // given
    let port = spawn_test_server();

    // when
    let create_word_dto = get_sample_create_word_dto();
    let create_response = post_word(port, &create_word_dto).await;
    let word_id = create_response.id;

    // then
    let word_list = get_word_list(port).await;
    assert_eq!(word_list.len(), 1);
    assert_eq!(word_list[0].translation, create_word_dto.translation);

    // when
    let update_word_dto = UpdateWordDto {
        word: create_word_dto.word,
        translation: Some("hola".to_owned()),
        source: Some("other source".to_owned()),
    };
    update_word(port, word_id, &update_word_dto).await;

    // then
    let word_list = get_word_list(port).await;
    assert_eq!(word_list.len(), 1);
    assert_eq!(word_list[0].translation, update_word_dto.translation);

    // when
    delete_word(port, word_id).await;

    // then
    let word_list = get_word_list(port).await;
    assert_eq!(word_list.len(), 0);
}

async fn get_word_list(port: u16) -> Vec<VocabResponseDto> {
    let client = reqwest::Client::new();
    let resp = client.get(format!("http://localhost:{port}/api/vocab"))
        .send()
        .await
        .unwrap();

    // then
    assert_eq!(resp.status(), 200);

    let mut resp_body: HashMap<String, Value> = resp.json().await.unwrap();
    assert_eq!(resp_body["status"], "success");
    let vocab_response: Vec<VocabResponseDto> = serde_json::from_value(resp_body.remove("words").unwrap())
        .expect("Deserialization failed");
    vocab_response
}


async fn post_word(port: u16, create_word_dto: &CreateWordDto) -> VocabResponseDto {
    let client = reqwest::Client::new();
    let resp = client.post(format!("http://localhost:{port}/api/vocab"))
        .json(&create_word_dto)
        .send()
        .await
        .unwrap();

    // then
    assert_eq!(resp.status(), 200);

    let mut resp_body: HashMap<String, Value> = resp.json().await.unwrap();
    assert_eq!(resp_body["status"], "success");
    let vocab_response: VocabResponseDto = serde_json::from_value(resp_body.remove("word").unwrap())
        .expect("Deserialization failed");
    assert_eq!(vocab_response.word, create_word_dto.word);
    assert_eq!(vocab_response.translation, create_word_dto.translation);
    assert_eq!(vocab_response.source, create_word_dto.source);
    vocab_response
}

async fn update_word(port: u16, word_id: i32, update_word_dto: &UpdateWordDto) {
    let client = reqwest::Client::new();
    let resp = client.put(format!("http://localhost:{port}/api/vocab/{word_id}"))
        .json(&update_word_dto)
        .send()
        .await
        .unwrap();

    // then
    assert_eq!(resp.status(), 200);

    let resp_body: HashMap<String, Value> = resp.json().await.unwrap();
    assert_eq!(resp_body["status"], "success");
}

async fn delete_word(port: u16, word_id: i32) {
    let client = reqwest::Client::new();
    let resp = client.delete(format!("http://localhost:{port}/api/vocab/{word_id}"))
        .send()
        .await
        .unwrap();

    // then
    assert_eq!(resp.status(), 200);

    let resp_body: HashMap<String, Value> = resp.json().await.unwrap();
    assert_eq!(resp_body["status"], "success");
}

fn get_sample_create_word_dto() -> CreateWordDto {
    CreateWordDto {
        word: "你好".to_owned(),
        translation: Some("hello".to_owned()),
        source: Some("some source".to_owned()),
    }
}

fn spawn_test_server() -> u16 {
    let database_url = "integration-test.db".to_owned();
    let mut connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    connection.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");
    clean_db_tables(&mut connection);

    let app = create_app(database_url);

    let server = axum::Server::bind(&"0.0.0.0:0".parse().unwrap())
        .serve(app.into_make_service());
    let port = server.local_addr().port();

    tokio::spawn(server);

    port
}

pub fn clean_db_tables(conn: &mut SqliteConnection) {
    let tables = ["words", "examples"];

    for table in &tables {
        diesel::sql_query(format!("DELETE FROM {}", table))
            .execute(conn)
            .expect(&format!("Error deleting from {}", table));
    }
}
