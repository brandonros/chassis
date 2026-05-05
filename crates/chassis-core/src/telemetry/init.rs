use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    config::{Config, LogFormat},
    telemetry::otel::build_provider,
};

pub fn init(config: &Config) -> Option<SdkTracerProvider> {
    let fmt_layer = match config.log_format {
        LogFormat::Json => tracing_subscriber::fmt::layer().json().boxed(),
        LogFormat::Pretty => tracing_subscriber::fmt::layer().boxed(),
    };

    let provider = build_provider(config);
    let otel_layer = provider.as_ref().map(|p| {
        let tracer = p.tracer(config.service_name.clone());
        tracing_opentelemetry::layer().with_tracer(tracer).boxed()
    });

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,tower_http=debug".into()))
        .with(fmt_layer)
        .with(otel_layer)
        .init();

    provider
}
