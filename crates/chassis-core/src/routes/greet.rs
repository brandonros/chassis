use axum::{
    Json,
    extract::{Path, State},
};
use metrics::counter;
use tracing::info;

use crate::{error::AppError, model::GreetResponse, state::AppState};

#[tracing::instrument(skip_all, fields(greeted = %name))]
pub async fn greet(
    State(_state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<GreetResponse>, AppError> {
    info!("greeting {}", name);
    if name.is_empty() {
        counter!("greet_invalid_total").increment(1);
        return Err(AppError::NotFound);
    }
    counter!("greet_success_total").increment(1);
    Ok(Json(GreetResponse {
        message: format!("Hello, {}!", name),
        name,
    }))
}
