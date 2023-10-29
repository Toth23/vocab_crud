use vocab_crud::create_app;

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

fn spawn_test_server() -> u16 {
    let database_url = "integration-test.db".to_owned();
    let app = create_app(database_url);

    let server = axum::Server::bind(&"0.0.0.0:0".parse().unwrap())
        .serve(app.into_make_service());
    let port = server.local_addr().port();

    tokio::spawn(server);

    port
}