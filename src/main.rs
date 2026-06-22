use anyhow::Context;
use std::net::SocketAddr;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use auto_platform_api::{config::Config, routes};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::from_env().context("failed to load config from environment")?;

    // Structured JSON logs — Cloud Run ships these straight to Cloud Logging.
    tracing_subscriber::registry()
        .with(EnvFilter::new(&cfg.rust_log))
        .with(fmt::layer().json())
        .init();

    let addr: SocketAddr = format!("{}:{}", cfg.host, cfg.port)
        .parse()
        .context("invalid HOST/PORT combination")?;

    let app = routes::build(cfg);

    tracing::info!(%addr, "auto-platform-api listening");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("failed to bind TCP listener")?;

    axum::serve(listener, app)
        .await
        .context("server error")?;

    Ok(())
}
