use core::fmt;
use std::{collections::HashMap, time::Duration};

use sqlx::types::Uuid;

use crate::{
    constants::{USER_SESSION_PREFIX, USER_SESSION_TTL_SECONDS},
    persistence::kv::KvProvider,
};

#[derive(Debug)]
pub enum SessionError {
    NotFound,
    StorageError(fred::error::Error),
    ParseError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserSession {
    pub token: String,
    pub user_id: Uuid,
    pub username: String,
    pub csrf_token: String,
}

impl From<UserSession> for HashMap<String, String> {
    fn from(value: UserSession) -> Self {
        let mut map = HashMap::new();
        map.insert("token".to_string(), value.token);
        map.insert("user_id".to_string(), value.user_id.to_string());
        map.insert("username".to_string(), value.username);
        map.insert("csrf_token".to_string(), value.csrf_token);
        return map;
    }
}

impl TryFrom<HashMap<String, String>> for UserSession {
    type Error = SessionError;

    fn try_from(mut map: HashMap<String, String>) -> Result<Self, SessionError> {
        let user_session = UserSession {
            token: map.remove("token").ok_or(SessionError::NotFound)?,
            user_id: map
                .remove("user_id")
                .ok_or(SessionError::NotFound)?
                .parse()
                .map_err(|_| SessionError::ParseError)?,
            username: map.remove("username").ok_or(SessionError::NotFound)?,
            csrf_token: map.remove("csrf_token").ok_or(SessionError::NotFound)?,
        };
        Ok(user_session)
    }
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::NotFound => write!(f, "session not found"),
            SessionError::ParseError => write!(f, "error parsing uuid"),
            SessionError::StorageError(e) => write!(f, "storage error: {}", e),
        }
    }
}

impl From<fred::error::Error> for SessionError {
    fn from(value: fred::error::Error) -> Self {
        SessionError::StorageError(value)
    }
}

pub trait SessionRepository {
    fn create_user_session(
        &self,
        user_session: UserSession,
    ) -> impl Future<Output = Result<(), SessionError>> + Send;
    fn get_user_session(
        &self,
        token: String,
    ) -> impl Future<Output = Result<UserSession, SessionError>> + Send;
    fn delete_user_session(
        &self,
        token: String,
    ) -> impl Future<Output = Result<(), SessionError>> + Send;
}

#[derive(Clone)]
pub struct SessionStore<K: KvProvider> {
    kv: K,
}

impl<K: KvProvider> SessionStore<K> {
    pub fn new(kv: K) -> Self {
        Self { kv }
    }
}

impl<K: KvProvider + Send + Sync> SessionRepository for SessionStore<K> {
    async fn create_user_session(&self, user_session: UserSession) -> Result<(), SessionError> {
        let key = format!("{}:{}", USER_SESSION_PREFIX, user_session.token);
        let ttl = Duration::from_secs(USER_SESSION_TTL_SECONDS);
        self.kv.hset(key.clone(), user_session.into()).await?;
        self.kv.expire(key, ttl).await?;
        Ok(())
    }

    async fn get_user_session(&self, token: String) -> Result<UserSession, SessionError> {
        let key = format!("{}:{}", USER_SESSION_PREFIX, token);
        let session = self.kv.hgetall(key).await?.try_into()?;
        Ok(session)
    }

    async fn delete_user_session(&self, token: String) -> Result<(), SessionError> {
        let key = format!("{}:{}", USER_SESSION_PREFIX, token);
        self.kv.del(key).await?;
        Ok(())
    }
}
