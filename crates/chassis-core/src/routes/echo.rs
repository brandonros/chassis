use axum::{Json, extract::State};
use tracing::info;

use crate::{
    auth::AuthenticatedSubject,
    error::AppError,
    model::{EchoRequest, EchoResponse},
    state::AppState,
};

#[tracing::instrument(skip_all, fields(subject = %subject.0.id, length = payload.message.len()))]
pub async fn echo(
    State(_state): State<AppState>,
    subject: AuthenticatedSubject,
    Json(payload): Json<EchoRequest>,
) -> Result<Json<EchoResponse>, AppError> {
    info!(
        "echoing message of {} chars for {}",
        payload.message.len(),
        subject.0.id
    );
    if payload.message.is_empty() {
        return Err(AppError::BadRequest("message must not be empty".into()));
    }
    Ok(Json(EchoResponse {
        length: payload.message.len(),
        echoed: payload.message,
    }))
}
