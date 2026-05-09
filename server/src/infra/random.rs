use std::fmt;

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use rand::{
    TryRng,
    rngs::{SysError, SysRng},
};

use crate::constants::{NONCE_SIZE_BYTES, SESSION_COOKIE_SIZE_BYTES};

pub enum RngServiceError {
    OsError(SysError),
}

impl From<SysError> for RngServiceError {
    fn from(value: SysError) -> Self {
        RngServiceError::OsError(value)
    }
}

impl fmt::Display for RngServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RngServiceError::OsError(e) => write!(f, "system error: {}", e),
        }
    }
}

pub trait RngService {
    fn generate_nonce(&mut self) -> Result<[u8; NONCE_SIZE_BYTES], RngServiceError>;
    fn generate_string(&mut self) -> Result<String, RngServiceError>;
}

#[derive(Clone)]
pub struct OsRngService {
    rng: SysRng,
}

impl OsRngService {
    pub fn new(rng: SysRng) -> Self {
        Self { rng }
    }
}

impl RngService for OsRngService {
    fn generate_nonce(&mut self) -> Result<[u8; NONCE_SIZE_BYTES], RngServiceError> {
        let mut key = [0u8; NONCE_SIZE_BYTES];
        self.rng.try_fill_bytes(&mut key)?;
        Ok(key)
    }

    fn generate_string(&mut self) -> Result<String, RngServiceError> {
        let mut key = [0u8; SESSION_COOKIE_SIZE_BYTES];
        self.rng.try_fill_bytes(&mut key)?;
        Ok(BASE64_URL_SAFE_NO_PAD.encode(&key))
    }
}
