use std::{fmt, sync::Arc, time::Duration};

use crate::{
    constants::{ANONYMOUS_CSRF_PREFIX, ANONYMOUS_CSRF_TTL_SECONDS},
    infra::persistence::kv::KvProvider,
};

#[derive(Debug)]
pub enum CsrfRepositoryError {
    StorageError(fred::error::Error),
}

impl fmt::Display for CsrfRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsrfRepositoryError::StorageError(e) => write!(f, "storage error: {}", e),
        }
    }
}

impl From<fred::error::Error> for CsrfRepositoryError {
    fn from(value: fred::error::Error) -> Self {
        CsrfRepositoryError::StorageError(value)
    }
}

pub trait CsrfRepository {
    fn create_anonymous_csrf(
        &self,
        token: String,
    ) -> impl Future<Output = Result<(), CsrfRepositoryError>> + Send;
    fn verify_anonymous_csrf(
        &self,
        token: String,
    ) -> impl Future<Output = Result<bool, CsrfRepositoryError>> + Send;
    fn delete_anonymous_csrf(
        &self,
        token: String,
    ) -> impl Future<Output = Result<(), CsrfRepositoryError>> + Send;
}

#[derive(Clone)]
pub struct CsrfStore<K: KvProvider> {
    kv: Arc<K>,
}

impl<K: KvProvider> CsrfStore<K> {
    pub fn new(kv: Arc<K>) -> Self {
        Self { kv }
    }
}

impl<K: KvProvider + Send + Sync> CsrfRepository for CsrfStore<K> {
    async fn create_anonymous_csrf(&self, token: String) -> Result<(), CsrfRepositoryError> {
        let key = format!("{}:{}", ANONYMOUS_CSRF_PREFIX, token);
        let ttl = Duration::from_secs(ANONYMOUS_CSRF_TTL_SECONDS);
        self.kv.set(key, "1".to_string(), Some(ttl)).await?;
        Ok(())
    }

    async fn verify_anonymous_csrf(&self, token: String) -> Result<bool, CsrfRepositoryError> {
        let key = format!("{}:{}", ANONYMOUS_CSRF_PREFIX, token);
        let csrf = self.kv.get(key).await?;
        Ok(csrf != None)
    }

    async fn delete_anonymous_csrf(&self, token: String) -> Result<(), CsrfRepositoryError> {
        let key = format!("{}:{}", ANONYMOUS_CSRF_PREFIX, token);
        self.kv.del(key).await?;
        Ok(())
    }
}
