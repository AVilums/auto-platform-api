use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use tracing::info;

use crate::{error::ApiError, models::EventBatch};

/// `POST /v1/events` — receives a batch of telemetry events from the launcher
/// and logs each one via `tracing`.  Cloud Run ships stdout/stderr to Cloud
/// Logging automatically, so structured JSON logs are enough for now.
pub async fn ingest(
    Json(batch): Json<EventBatch>,
) -> Result<impl IntoResponse, ApiError> {
    if batch.events.is_empty() {
        return Err(ApiError::BadRequest("events array must not be empty".into()));
    }

    for event in &batch.events {
        info!(
            event_id    = %event.id,
            event_type  = ?event.event_type,
            tool        = event.tool_name.as_deref().unwrap_or("-"),
            version     = event.tool_version.as_deref().unwrap_or("-"),
            status      = event.status.as_deref().unwrap_or("-"),
            details     = event.details.as_deref().unwrap_or("-"),
            timestamp   = %event.timestamp,
            "telemetry event received",
        );
    }

    let count = batch.events.len();
    Ok((
        StatusCode::ACCEPTED,
        Json(json!({ "accepted": count })),
    ))
}
