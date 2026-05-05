pub mod auth;
pub mod config;
pub mod error;
pub mod model;
pub mod router;
pub mod routes;
pub mod server;
pub mod shutdown;
pub mod state;
pub mod telemetry;

use std::sync::Arc;

use axum_prometheus::PrometheusMetricLayerBuilder;

use crate::{auth::JwtAuthenticator, config::Config, state::AppState};

pub async fn run() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    let tracer_provider = telemetry::init(&config);

    let (prometheus_layer, prometheus_handle) = PrometheusMetricLayerBuilder::new()
        .with_ignore_patterns(&["/livez", "/readyz"])
        .with_default_metrics()
        .build_pair();

    let mut auth = JwtAuthenticator::new_hs256(config.jwt_secret.as_bytes());
    if let Some(issuer) = &config.jwt_issuer {
        auth = auth.with_issuer(issuer);
    }
    if let Some(audience) = &config.jwt_audience {
        auth = auth.with_audience(audience);
    }
    let state = AppState::new(Arc::new(auth));

    let app_router = router::build(state, prometheus_layer, &config);
    let admin_router = router::build_admin(prometheus_handle);

    let result = server::run(
        &config.bind_addr,
        app_router,
        &config.admin_bind_addr,
        admin_router,
    )
    .await;

    if let Some(provider) = tracer_provider
        && let Err(e) = provider.shutdown()
    {
        tracing::warn!(error = ?e, "tracer provider shutdown error");
    }

    result
}
