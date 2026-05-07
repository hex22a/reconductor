use csrf::{AesGcmCsrfProtection, CsrfError, CsrfProtection};
use std::fmt;

use crate::infra::random::{OsRngService, RngService, RngServiceError};

pub enum CsrfServiceError {
    NotGenerated(String),
    RngError(RngServiceError),
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

impl fmt::Display for CsrfServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsrfServiceError::NotGenerated(e) => write!(f, "csrf token not generated: {}", e),
            CsrfServiceError::RngError(e) => write!(f, "error generating random value: {}", e),
            CsrfServiceError::CsrfInternalError => write!(f, "csrf internal error"),
        }
    }
}

pub trait CsrfService {
    fn generate(&mut self, ttl: u64) -> Result<(String, String), CsrfServiceError>;
    fn verify(&self, token: &str, cookie: &str) -> bool;
}

struct AesGcmCsrfService {
    protect: AesGcmCsrfProtection,
    rng: OsRngService,
}

impl AesGcmCsrfService {
    fn new(rng: OsRngService, key: [u8; 32]) -> Self {
        Self {
            protect: AesGcmCsrfProtection::from_key(key),
            rng,
        }
    }
}

impl CsrfService for AesGcmCsrfService {
    fn generate(&mut self, ttl: u64) -> Result<(String, String), CsrfServiceError> {
        let nonce = self.rng.generate_nonce()?;
        let (token, cookie) = self.protect.generate_token_pair(Some(&nonce), ttl as i64)?;
        Ok((token.b64_string(), cookie.b64_string()))
    }

    fn verify(&self, token: &str, cookie: &str) -> bool {
        let token = match self.protect.parse_token(token.as_bytes()) {
            Ok(t) => t,
            Err(_) => return false,
        };
        let cookie = match self.protect.parse_cookie(cookie.as_bytes()) {
            Ok(t) => t,
            Err(_) => return false,
        };
        self.protect.verify_token_pair(&token, &cookie).is_ok()
    }
}
