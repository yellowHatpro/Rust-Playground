use axum::Json;
use axum::response::IntoResponse;

pub async fn healthcheck_handler() -> impl IntoResponse {
    const MESSAGE: &str = "raxum server, created to fafo rust backend dev";
    let res = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });
    Json(res)
}