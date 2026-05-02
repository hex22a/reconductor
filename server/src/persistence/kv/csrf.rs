use std::{fmt, time::Duration};

use crate::{
    constants::{ANONYMOUS_CSRF_PREFIX, ANONYMOUS_CSRF_TTL_SECONDS},
    persistence::kv::KvProvider,
};

#[derive(Debug)]
pub enum CsrfError {
    StorageError(fred::error::Error),
}

impl fmt::Display for CsrfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsrfError::StorageError(e) => write!(f, "storage error: {}", e),
        }
    }
}

impl From<fred::error::Error> for CsrfError {
    fn from(value: fred::error::Error) -> Self {
        CsrfError::StorageError(value)
    }
}

#[allow(async_fn_in_trait)]
pub trait CsrfRepository {
    async fn create_anonymous_csrf(&self, token: &str) -> Result<(), CsrfError>;
    async fn verify_anonymous_csrf(&self, token: &str) -> Result<bool, CsrfError>;
    async fn delete_anonymous_csrf(&self, token: &str) -> Result<(), CsrfError>;
}

pub struct CsrfStore<K: KvProvider> {
    kv: K,
}

impl<K: KvProvider> CsrfStore<K> {
    pub fn new(kv: K) -> Self {
        Self { kv }
    }
}

impl<K: KvProvider> CsrfRepository for CsrfStore<K> {
    async fn create_anonymous_csrf(&self, token: &str) -> Result<(), CsrfError> {
        let key = format!("{}:{}", ANONYMOUS_CSRF_PREFIX, token);
        let ttl = Duration::from_secs(ANONYMOUS_CSRF_TTL_SECONDS);
        self.kv.set(&key, "1", Some(ttl)).await?;
        Ok(())
    }

    async fn verify_anonymous_csrf(&self, token: &str) -> Result<bool, CsrfError> {
        let key = format!("{}:{}", ANONYMOUS_CSRF_PREFIX, token);
        let csrf = self.kv.get(&key).await?;
        Ok(csrf != None)
    }

    async fn delete_anonymous_csrf(&self, token: &str) -> Result<(), CsrfError> {
        let key = format!("{}:{}", ANONYMOUS_CSRF_PREFIX, token);
        self.kv.del(&key).await?;
        Ok(())
    }
}
