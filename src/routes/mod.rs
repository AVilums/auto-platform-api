pub mod events;
pub mod health;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{auth::require_api_key, config::Config};

/// Build the full application router.
///
/// Structure:
/// - `GET  /health`      — unauthenticated liveness probe
/// - `POST /v1/events`   — authenticated telemetry ingest
pub fn build(cfg: Config) -> Router {
    // Routes that require a valid API key
    let protected = Router::new()
        .route("/v1/events", post(events::ingest))
        .layer(middleware::from_fn_with_state(
            cfg.clone(),
            require_api_key,
        ));

    Router::new()
        .route("/health", get(health::health))
        .merge(protected)
        .with_state(cfg)
}
