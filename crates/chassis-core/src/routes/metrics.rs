use axum::extract::State;
use metrics_exporter_prometheus::PrometheusHandle;

use crate::error::AppError;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn metrics(State(handle): State<PrometheusHandle>) -> Result<String, AppError> {
    Ok(handle.render())
}
