use base64::{Engine, engine::general_purpose};
use csrf::{AesGcmCsrfProtection, CsrfError, CsrfProtection};
use std::sync::Arc;
use thiserror::Error;

#[cfg(test)]
use mockall::automock;

use crate::{
    domain::{csrf_cookie::CsrfCookie, csrf_token::CsrfToken},
    infra::random::{OsRngService, RngService, RngServiceError},
};

#[derive(Debug, Clone, Error)]
pub enum CsrfServiceError {
    #[error("csrf token not generated: {0}")]
    NotGenerated(String),

    #[error("error generating random value: {0}")]
    RngError(#[source] RngServiceError),

    #[error("csrf internal error")]
    CsrfInternalError,
}

impl From<csrf::CsrfError> for CsrfServiceError {
    fn from(e: csrf::CsrfError) -> Self {
        match e {
            CsrfError::ValidationFailure(msg) => CsrfServiceError::NotGenerated(msg),
            CsrfError::EncryptionFailure(msg) => CsrfServiceError::NotGenerated(msg),
            CsrfError::InternalError => CsrfServiceError::CsrfInternalError,
        }
    }
}

impl From<RngServiceError> for CsrfServiceError {
    fn from(value: RngServiceError) -> Self {
        CsrfServiceError::RngError(value)
    }
}

#[cfg_attr(test, automock)]
pub trait CsrfService {
    fn generate(&self, ttl: u64) -> Result<(CsrfToken, CsrfCookie), CsrfServiceError>;
    fn verify(&self, token: &CsrfToken, cookie: &CsrfCookie) -> bool;
}

#[derive(Clone)]
pub struct AesGcmCsrfService {
    protect: AesGcmCsrfProtection,
    rng: Arc<OsRngService>,
}

impl AesGcmCsrfService {
    pub fn new(rng: Arc<OsRngService>, key: [u8; 32]) -> Self {
        Self {
            protect: AesGcmCsrfProtection::from_key(key),
            rng,
        }
    }
}

impl CsrfService for AesGcmCsrfService {
    fn generate(&self, ttl: u64) -> Result<(CsrfToken, CsrfCookie), CsrfServiceError> {
        let nonce = self.rng.generate_nonce()?;
        let (token, cookie) = self.protect.generate_token_pair(Some(&nonce), ttl as i64)?;
        Ok((
            CsrfToken::new(token.b64_string()),
            CsrfCookie::new(cookie.b64_string()),
        ))
    }

    fn verify(&self, token: &CsrfToken, cookie: &CsrfCookie) -> bool {
        let token_bytes = match general_purpose::STANDARD.decode(token.as_ref()) {
            Ok(b) => b,
            Err(_) => return false,
        };
        let cookie_bytes = match general_purpose::STANDARD.decode(cookie.as_ref()) {
            Ok(b) => b,
            Err(_) => return false,
        };
        let token = match self.protect.parse_token(&token_bytes) {
            Ok(t) => t,
            Err(_) => return false,
        };
        let cookie = match self.protect.parse_cookie(&cookie_bytes) {
            Ok(t) => t,
            Err(_) => return false,
        };
        self.protect.verify_token_pair(&token, &cookie).is_ok()
    }
}
