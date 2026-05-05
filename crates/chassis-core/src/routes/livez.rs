use axum::{extract::State, http::StatusCode};

use crate::{error::AppError, state::AppState};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn livez(State(_state): State<AppState>) -> Result<StatusCode, AppError> {
    Ok(StatusCode::OK)
}
