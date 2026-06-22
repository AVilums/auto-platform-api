// Re-export modules used by integration tests so they can reference
// `auto_platform_api::config::Config` and `auto_platform_api::routes`.
pub mod auth;
pub mod config;
pub mod error;
pub mod models;
pub mod routes;
