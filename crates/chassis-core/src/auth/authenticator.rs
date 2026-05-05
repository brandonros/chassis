use crate::auth::Subject;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("invalid token")]
    InvalidToken,
}

pub trait Authenticator: Send + Sync + 'static {
    fn authenticate(
        &self,
        token: &str,
    ) -> impl std::future::Future<Output = Result<Subject, AuthError>> + Send;
}
