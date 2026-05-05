use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use axum_prometheus::PrometheusMetricLayer;
use metrics_exporter_prometheus::PrometheusHandle;
use tower_http::{
    compression::CompressionLayer,
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::{
    config::Config,
    routes::{
        echo::echo, greet::greet, hello::hello, livez::livez, metrics::metrics,
        not_found::not_found, readyz::readyz,
    },
    state::AppState,
    telemetry::MakeSpanWithRequestId,
};

pub fn build(
    state: AppState,
    prometheus_layer: PrometheusMetricLayer<'static>,
    config: &Config,
) -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(MakeSpanWithRequestId)
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let v1_router = Router::new()
        .route("/", get(hello))
        .route("/greet/{name}", get(greet))
        .route("/echo", post(echo));

    Router::new()
        .nest("/v1", v1_router)
        .route("/livez", get(livez))
        .route("/readyz", get(readyz))
        .fallback(not_found)
        .with_state(state)
        .layer(prometheus_layer)
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(trace_layer)
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            config.request_timeout,
        ))
        .layer(RequestBodyLimitLayer::new(config.body_limit_bytes))
        .layer(CompressionLayer::new())
}

pub fn build_admin(prometheus_handle: PrometheusHandle) -> Router {
    Router::new()
        .route("/metrics", get(metrics))
        .with_state(prometheus_handle)
}
