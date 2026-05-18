use std::{
    fmt,
    sync::{Arc, Mutex},
};

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use rand::{
    TryRng,
    rngs::{SysError, SysRng},
};

use crate::constants::{NONCE_SIZE_BYTES, SESSION_ID_SIZE_BYTES};

pub(crate) enum RngServiceError {
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

pub(crate) trait RngService {
    fn generate_nonce(&self) -> Result<[u8; NONCE_SIZE_BYTES], RngServiceError>;
    fn generate_session_id(&self) -> Result<String, RngServiceError>;
}

#[derive(Clone)]
pub(crate) struct OsRngService {
    rng: Arc<Mutex<SysRng>>,
}

impl OsRngService {
    pub(crate) fn new(rng: Arc<Mutex<SysRng>>) -> Self {
        Self { rng }
    }
}

impl RngService for OsRngService {
    fn generate_nonce(&self) -> Result<[u8; NONCE_SIZE_BYTES], RngServiceError> {
        let mut key = [0u8; NONCE_SIZE_BYTES];
        self.rng.lock().unwrap().try_fill_bytes(&mut key)?;
        Ok(key)
    }

    fn generate_session_id(&self) -> Result<String, RngServiceError> {
        let mut key = [0u8; SESSION_ID_SIZE_BYTES];
        self.rng.lock().unwrap().try_fill_bytes(&mut key)?;
        Ok(BASE64_URL_SAFE_NO_PAD.encode(key))
    }
}
