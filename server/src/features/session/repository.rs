use std::{collections::HashMap, sync::Arc, time::Duration};

#[cfg(test)]
use mockall::automock;
use secrecy::{ExposeSecret, SecretString};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    constants::{USER_SESSION_PREFIX, USER_SESSION_TTL_SECONDS},
    domain::secret_types::SessionToken,
    features::session::model::UserSession,
    infra::persistence::kv::{KvProvider, StorageId},
};

#[derive(Debug, Error)]
pub enum SessionRepositoryError {
    #[error("session not found")]
    NotFound,

    #[error("storage error: {0}")]
    StorageError(#[source] fred::error::Error),

    #[error("error parsing uuid")]
    ParseError,
}

impl From<UserSession> for HashMap<String, String> {
    fn from(value: UserSession) -> Self {
        let mut map = HashMap::new();
        map.insert(
            String::from("token"),
            value.token.into_inner().expose_secret().to_owned(),
        );
        map.insert(
            String::from("user_id"),
            value.user_id.into_inner().to_string(),
        );
        map.insert(String::from("username"), value.username.into_inner());
        map.insert(
            String::from("csrf_token"),
            value.csrf_token.into_inner().expose_secret().to_owned(),
        );
        map.insert(
            String::from("csrf_cookie"),
            value.csrf_cookie.into_inner().expose_secret().to_owned(),
        );
        map
    }
}

impl TryFrom<HashMap<String, String>> for UserSession {
    type Error = SessionRepositoryError;

    fn try_from(mut map: HashMap<String, String>) -> Result<Self, SessionRepositoryError> {
        let user_session = UserSession {
            token: map
                .remove("token")
                .map(SecretString::from)
                .ok_or(SessionRepositoryError::NotFound)?
                .into(),
            user_id: map
                .remove("user_id")
                .ok_or(SessionRepositoryError::NotFound)?
                .parse::<Uuid>()
                .map_err(|_| SessionRepositoryError::ParseError)?
                .into(),
            username: map
                .remove("username")
                .ok_or(SessionRepositoryError::NotFound)?
                .into(),
            csrf_token: map
                .remove("csrf_token")
                .map(SecretString::from)
                .ok_or(SessionRepositoryError::NotFound)?
                .into(),
            csrf_cookie: map
                .remove("csrf_cookie")
                .map(SecretString::from)
                .ok_or(SessionRepositoryError::NotFound)?
                .into(),
        };
        Ok(user_session)
    }
}

impl From<fred::error::Error> for SessionRepositoryError {
    fn from(value: fred::error::Error) -> Self {
        SessionRepositoryError::StorageError(value)
    }
}

#[cfg_attr(test, automock)]
pub trait SessionRepository {
    fn create_user_session(
        &self,
        user_session: UserSession,
    ) -> impl Future<Output = Result<(), SessionRepositoryError>> + Send;
    fn get_user_session(
        &self,
        token: &SessionToken,
    ) -> impl Future<Output = Result<UserSession, SessionRepositoryError>> + Send;
    fn delete_user_session(
        &self,
        token: &SessionToken,
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
        let key = format!(
            "{}:{}",
            USER_SESSION_PREFIX,
            user_session.token.storage_id()
        );
        let ttl = Duration::from_secs(USER_SESSION_TTL_SECONDS);
        self.kv.hset(&key, user_session.into()).await?;
        self.kv.expire(&key, &ttl).await?;
        Ok(())
    }

    async fn get_user_session(
        &self,
        token: &SessionToken,
    ) -> Result<UserSession, SessionRepositoryError> {
        let key = format!("{}:{}", USER_SESSION_PREFIX, token.storage_id());
        let session = self.kv.hgetall(&key).await?.try_into()?;
        Ok(session)
    }

    async fn delete_user_session(
        &self,
        token: &SessionToken,
    ) -> Result<(), SessionRepositoryError> {
        let key = format!("{}:{}", USER_SESSION_PREFIX, token.storage_id());
        self.kv.del(&key).await?;
        Ok(())
    }
}
