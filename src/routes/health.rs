use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

/// `GET /health` — liveness probe used by Cloud Run and load balancers.
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}
