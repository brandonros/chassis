use axum::http::Request;
use tower_http::trace::MakeSpan;
use tracing::{Span, info_span};

#[derive(Clone)]
pub struct MakeSpanWithRequestId;

impl<B> MakeSpan<B> for MakeSpanWithRequestId {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let request_id = request
            .headers()
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");

        info_span!(
            "http_request",
            method = %request.method(),
            uri = %request.uri(),
            request_id = %request_id,
        )
    }
}
