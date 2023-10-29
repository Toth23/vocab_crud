use vocab_crud::create_app;

#[tokio::test]
async fn test_get_endpoint() {
    let database_url = "vocab.db".to_owned();
    let app = create_app(database_url);

    let server = axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service());

    let _ = tokio::spawn(server);

    let client = reqwest::Client::new();
    let resp = client.get("http://localhost:3000/api/health")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}