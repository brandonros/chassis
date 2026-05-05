use opentelemetry::KeyValue;
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{Resource, trace::SdkTracerProvider};

use crate::config::Config;

pub fn build_provider(config: &Config) -> Option<SdkTracerProvider> {
    let endpoint = config.otlp_endpoint.as_ref()?;

    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()
        .ok()?;

    let resource = Resource::builder()
        .with_attribute(KeyValue::new("service.name", config.service_name.clone()))
        .build();

    Some(
        SdkTracerProvider::builder()
            .with_resource(resource)
            .with_batch_exporter(exporter)
            .build(),
    )
}
