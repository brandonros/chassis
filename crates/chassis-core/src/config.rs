use std::time::Duration;

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: String,
    pub admin_bind_addr: String,
    pub log_format: LogFormat,
    pub request_timeout: Duration,
    pub body_limit_bytes: usize,
    pub otlp_endpoint: Option<String>,
    pub service_name: String,
    pub jwt_secret: String,
    pub jwt_issuer: Option<String>,
    pub jwt_audience: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    Pretty,
    Json,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            bind_addr: std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into()),
            admin_bind_addr: std::env::var("ADMIN_BIND_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:9090".into()),
            log_format: match std::env::var("LOG_FORMAT").as_deref() {
                Ok("json") => LogFormat::Json,
                _ => LogFormat::Pretty,
            },
            request_timeout: Duration::from_secs(
                std::env::var("REQUEST_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            body_limit_bytes: std::env::var("BODY_LIMIT_BYTES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1024 * 1024),
            otlp_endpoint: std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok(),
            service_name: std::env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| "chassis".into()),
            jwt_secret: std::env::var("JWT_SECRET").context("JWT_SECRET is required")?,
            jwt_issuer: std::env::var("JWT_ISSUER").ok(),
            jwt_audience: std::env::var("JWT_AUDIENCE").ok(),
        })
    }
}
