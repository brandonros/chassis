use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::Deserialize;

use crate::auth::{AuthError, Authenticator, Subject};

#[derive(Deserialize)]
struct Claims {
    sub: String,
    #[serde(default)]
    scope: Option<String>,
    #[serde(default)]
    scopes: Option<Vec<String>>,
    #[serde(default)]
    scp: Option<Vec<String>>,
}

pub struct JwtAuthenticator {
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtAuthenticator {
    pub fn new_hs256(secret: &[u8]) -> Self {
        Self {
            decoding_key: DecodingKey::from_secret(secret),
            validation: lenient_validation(Algorithm::HS256),
        }
    }

    pub fn new_rs256(public_key_pem: &[u8]) -> Result<Self, jsonwebtoken::errors::Error> {
        Ok(Self {
            decoding_key: DecodingKey::from_rsa_pem(public_key_pem)?,
            validation: lenient_validation(Algorithm::RS256),
        })
    }

    pub fn with_issuer(mut self, issuer: &str) -> Self {
        self.validation.set_issuer(&[issuer]);
        self
    }

    pub fn with_audience(mut self, audience: &str) -> Self {
        self.validation.set_audience(&[audience]);
        self.validation.validate_aud = true;
        self
    }
}

fn lenient_validation(algorithm: Algorithm) -> Validation {
    let mut v = Validation::new(algorithm);
    // jsonwebtoken defaults validate_aud=true, which rejects tokens carrying an `aud`
    // claim unless an expected audience is configured. Disable until with_audience() opts in.
    v.validate_aud = false;
    v
}

impl Authenticator for JwtAuthenticator {
    async fn authenticate(&self, token: &str) -> Result<Subject, AuthError> {
        let token_data =
            decode::<Claims>(token, &self.decoding_key, &self.validation).map_err(|e| {
                tracing::debug!(error = %e, "jwt validation failed");
                AuthError::InvalidToken
            })?;

        let scopes = extract_scopes(&token_data.claims);

        Ok(Subject {
            id: token_data.claims.sub,
            scopes,
        })
    }
}

fn extract_scopes(claims: &Claims) -> Vec<String> {
    if let Some(s) = &claims.scope {
        return s.split_whitespace().map(String::from).collect();
    }
    if let Some(v) = &claims.scopes {
        return v.clone();
    }
    if let Some(v) = &claims.scp {
        return v.clone();
    }
    Vec::new()
}
