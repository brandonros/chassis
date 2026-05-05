use std::sync::Arc;

use crate::auth::JwtAuthenticator;

#[derive(Clone)]
pub struct AppState {
    pub authenticator: Arc<JwtAuthenticator>,
}

impl AppState {
    pub fn new(authenticator: Arc<JwtAuthenticator>) -> Self {
        Self { authenticator }
    }
}
