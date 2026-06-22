use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::{config::Config, error::ApiError};

/// Axum middleware that validates the `X-API-Key` header against the
/// configured `API_MASTER_KEY`.  Requests without a matching key receive 401.
pub async fn require_api_key(
    State(cfg): State<Config>,
    request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let key = request
        .headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if key != cfg.api_master_key {
        return Err(ApiError::Unauthorised);
    }

    Ok(next.run(request).await)
}
