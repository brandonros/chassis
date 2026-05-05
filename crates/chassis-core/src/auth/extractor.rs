use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{
    auth::{Authenticator, Subject},
    error::AppError,
    state::AppState,
};

pub struct AuthenticatedSubject(pub Subject);

impl FromRequestParts<AppState> for AuthenticatedSubject {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .ok_or_else(|| {
                tracing::debug!("missing or malformed Authorization header");
                AppError::Unauthorized
            })?;

        state
            .authenticator
            .authenticate(token)
            .await
            .map(AuthenticatedSubject)
            .map_err(|e| {
                tracing::debug!(error = %e, "authentication failed");
                AppError::Unauthorized
            })
    }
}
