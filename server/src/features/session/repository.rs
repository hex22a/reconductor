use core::fmt;
use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::{
    constants::{USER_SESSION_PREFIX, USER_SESSION_TTL_SECONDS},
    features::session::model::UserSession,
    infra::persistence::kv::KvProvider,
};

#[derive(Debug)]
pub enum SessionRepositoryError {
    NotFound,
    StorageError(fred::error::Error),
    ParseError,
}

impl From<UserSession> for HashMap<String, String> {
    fn from(value: UserSession) -> Self {
        let mut map = HashMap::new();
        map.insert("token".to_string(), value.token);
        map.insert("user_id".to_string(), value.user_id.to_string());
        map.insert("username".to_string(), value.username);
        map.insert("csrf_token".to_string(), value.csrf_token);
        map.insert("csrf_cookie".to_string(), value.csrf_cookie);
        map
    }
}

impl TryFrom<HashMap<String, String>> for UserSession {
    type Error = SessionRepositoryError;

    fn try_from(mut map: HashMap<String, String>) -> Result<Self, SessionRepositoryError> {
        let user_session = UserSession {
            token: map
                .remove("token")
                .ok_or(SessionRepositoryError::NotFound)?,
            user_id: map
                .remove("user_id")
                .ok_or(SessionRepositoryError::NotFound)?
                .parse()
                .map_err(|_| SessionRepositoryError::ParseError)?,
            username: map
                .remove("username")
                .ok_or(SessionRepositoryError::NotFound)?,
            csrf_token: map
                .remove("csrf_token")
                .ok_or(SessionRepositoryError::NotFound)?,
            csrf_cookie: map
                .remove("csrf_cookie")
                .ok_or(SessionRepositoryError::NotFound)?,
        };
        Ok(user_session)
    }
}

impl fmt::Display for SessionRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionRepositoryError::NotFound => write!(f, "session not found"),
            SessionRepositoryError::ParseError => write!(f, "error parsing uuid"),
            SessionRepositoryError::StorageError(e) => write!(f, "storage error: {}", e),
        }
    }
}

impl From<fred::error::Error> for SessionRepositoryError {
    fn from(value: fred::error::Error) -> Self {
        SessionRepositoryError::StorageError(value)
    }
}

pub trait SessionRepository {
    fn create_user_session(
        &self,
        user_session: UserSession,
    ) -> impl Future<Output = Result<(), SessionRepositoryError>> + Send;
    fn get_user_session(
        &self,
        token: &str,
    ) -> impl Future<Output = Result<UserSession, SessionRepositoryError>> + Send;
    fn delete_user_session(
        &self,
        token: &str,
    ) -> impl Future<Output = Result<(), SessionRepositoryError>> + Send;
}

#[derive(Clone)]
pub struct SessionStore<K: KvProvider> {
    kv: Arc<K>,
}

impl<K: KvProvider> SessionStore<K> {
    pub fn new(kv: Arc<K>) -> Self {
        Self { kv }
    }
}

impl<K: KvProvider + Send + Sync> SessionRepository for SessionStore<K> {
    async fn create_user_session(
        &self,
        user_session: UserSession,
    ) -> Result<(), SessionRepositoryError> {
        let key = format!("{}:{}", USER_SESSION_PREFIX, user_session.token);
        let ttl = Duration::from_secs(USER_SESSION_TTL_SECONDS);
        self.kv.hset(&key, user_session.into()).await?;
        self.kv.expire(&key, &ttl).await?;
        Ok(())
    }

    async fn get_user_session(&self, token: &str) -> Result<UserSession, SessionRepositoryError> {
        let key = format!("{}:{}", USER_SESSION_PREFIX, token);
        let session = self.kv.hgetall(&key).await?.try_into()?;
        Ok(session)
    }

    async fn delete_user_session(&self, token: &str) -> Result<(), SessionRepositoryError> {
        let key = format!("{}:{}", USER_SESSION_PREFIX, token);
        self.kv.del(&key).await?;
        Ok(())
    }
}
