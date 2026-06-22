//! Integration tests for the auto-platform-api HTTP surface.
//!
//! These tests spin up the real axum router (no network, no OS port) using
//! `axum-test`, so they are fast and require no external services.

use auto_platform_api::routes;
use axum_test::TestServer;
use serde_json::json;

const TEST_KEY: &str = "integration-test-key";

fn test_server() -> TestServer {
    let cfg = auto_platform_api::config::Config {
        host: "127.0.0.1".into(),
        port: 8080,
        api_master_key: TEST_KEY.into(),
        rust_log: "error".into(),
    };
    let app = routes::build(cfg);
    TestServer::new(app).expect("failed to create test server")
}

// ── /health ──────────────────────────────────────────────────────────────────

#[tokio::test]
async fn health_returns_200() {
    let server = test_server();
    let resp = server.get("/health").await;
    resp.assert_status_ok();
    resp.assert_json(&json!({ "status": "ok" }));
}

// ── POST /v1/events — auth ────────────────────────────────────────────────────

#[tokio::test]
async fn events_rejects_missing_key() {
    let server = test_server();
    let resp = server
        .post("/v1/events")
        .json(&json!({ "events": [] }))
        .await;
    resp.assert_status_unauthorized();
}

#[tokio::test]
async fn events_rejects_wrong_key() {
    let server = test_server();
    let resp = server
        .post("/v1/events")
        .add_header("x-api-key", "wrong-key")
        .json(&json!({ "events": [] }))
        .await;
    resp.assert_status_unauthorized();
}

// ── POST /v1/events — payload validation ─────────────────────────────────────

#[tokio::test]
async fn events_rejects_empty_batch() {
    let server = test_server();
    let resp = server
        .post("/v1/events")
        .add_header("x-api-key", TEST_KEY)
        .json(&json!({ "events": [] }))
        .await;
    resp.assert_status(axum::http::StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn events_rejects_malformed_json() {
    let server = test_server();
    let resp = server
        .post("/v1/events")
        .add_header("x-api-key", TEST_KEY)
        .text("not json at all")
        .await;
    // axum returns 415 (unsupported media type) when content-type is missing
    assert!(
        resp.status_code().is_client_error(),
        "expected 4xx, got {}",
        resp.status_code()
    );
}

// ── POST /v1/events — happy path ──────────────────────────────────────────────

#[tokio::test]
async fn events_accepts_valid_batch() {
    let server = test_server();

    let payload = json!({
        "events": [
            {
                "id": "evt-001",
                "timestamp": "2024-06-01T10:00:00Z",
                "event_type": "ExecutionSuccess",
                "tool_name": "data-processor",
                "tool_version": "1.2.0",
                "status": "ok",
                "details": null
            },
            {
                "id": "evt-002",
                "timestamp": "2024-06-01T10:00:01Z",
                "event_type": "DownloadComplete",
                "tool_name": "report-generator",
                "tool_version": "2.0.0",
                "status": null,
                "details": "downloaded 3 MB"
            }
        ]
    });

    let resp = server
        .post("/v1/events")
        .add_header("x-api-key", TEST_KEY)
        .json(&payload)
        .await;

    resp.assert_status(axum::http::StatusCode::ACCEPTED);
    resp.assert_json(&json!({ "accepted": 2 }));
}

#[tokio::test]
async fn events_accepts_unknown_event_type() {
    let server = test_server();

    let payload = json!({
        "events": [{
            "id": "evt-future",
            "timestamp": "2025-01-01T00:00:00Z",
            "event_type": "BrandNewEventFromFuture",
            "tool_name": null,
            "tool_version": null,
            "status": null,
            "details": null
        }]
    });

    let resp = server
        .post("/v1/events")
        .add_header("x-api-key", TEST_KEY)
        .json(&payload)
        .await;

    resp.assert_status(axum::http::StatusCode::ACCEPTED);
    resp.assert_json(&json!({ "accepted": 1 }));
}
