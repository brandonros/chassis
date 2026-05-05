use axum::extract::State;
use tracing::info;

use crate::{error::AppError, state::AppState};

#[tracing::instrument(skip_all)]
pub async fn hello(State(_state): State<AppState>) -> Result<&'static str, AppError> {
    info!("handling /");
    Ok("Hello, World!")
}
