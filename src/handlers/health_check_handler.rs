use axum::Json;
use axum::response::IntoResponse;

pub async fn check_health() -> impl IntoResponse {
    const MESSAGE: &str = "It's running!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
