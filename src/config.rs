use anyhow::Result;
use serde::Deserialize;

/// Application configuration, loaded from environment variables.
///
/// Local: export vars or use a `.env` file (loaded by `dotenvy`).
/// Cloud Run: set vars directly in the service definition.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// Interface to bind — always `0.0.0.0` on Cloud Run.
    #[serde(default = "default_host")]
    pub host: String,

    /// Port to listen on.
    #[serde(default = "default_port")]
    pub port: u16,

    /// Shared secret the launcher must include as `X-API-Key`.
    pub api_master_key: String,

    /// `RUST_LOG`-style filter string forwarded to tracing-subscriber.
    #[serde(default = "default_log_level")]
    pub rust_log: String,
}

impl Config {
    /// Load config from environment, optionally picking up a `.env` file first.
    pub fn from_env() -> Result<Self> {
        // ignore errors — file simply may not exist
        let _ = dotenvy::dotenv();
        let cfg = envy::from_env::<Config>()?;
        Ok(cfg)
    }
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_log_level() -> String {
    "info".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn loads_required_fields() {
        env::set_var("API_MASTER_KEY", "test-key-abc");
        let cfg = Config::from_env().expect("should load");
        assert_eq!(cfg.api_master_key, "test-key-abc");
        assert_eq!(cfg.port, 8080);
        assert_eq!(cfg.host, "127.0.0.1");
    }

    #[test]
    fn defaults_applied() {
        env::set_var("API_MASTER_KEY", "some-key");
        let cfg = Config::from_env().unwrap();
        assert_eq!(cfg.rust_log, "info");
    }
}
