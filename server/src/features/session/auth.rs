use std::sync::Arc;

use crate::{
    features::session::{error::SessionError, model::UserSession},
    infra::persistence::kv::session::SessionRepository,
};

pub trait AuthFeature {
    fn auth(
        &self,
        session_id: String,
    ) -> impl Future<Output = Result<UserSession, SessionError>> + Send;
}

#[derive(Clone)]
pub struct UserAuthFeature<S: SessionRepository> {
    session_repository: Arc<S>,
}

impl<S: SessionRepository> UserAuthFeature<S> {
    pub fn new(session_repository: Arc<S>) -> Self {
        UserAuthFeature { session_repository }
    }
}

impl<S> AuthFeature for UserAuthFeature<S>
where
    S: SessionRepository + Send + Sync,
{
    async fn auth(&self, session_id: String) -> Result<UserSession, SessionError> {
        Ok(self.session_repository.get_user_session(session_id).await?)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use std::sync::Mutex;

    use crate::{
        features::session::model::UserSession,
        infra::persistence::kv::session::{SessionRepository, SessionRepositoryError},
    };

    struct MockSessionRepository {
        error: Mutex<Option<SessionRepositoryError>>,
        return_value: UserSession,
    }
    impl SessionRepository for MockSessionRepository {
        async fn create_user_session(&self, _: UserSession) -> Result<(), SessionRepositoryError> {
            todo!()
        }

        async fn get_user_session(&self, _: String) -> Result<UserSession, SessionRepositoryError> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(self.return_value.clone()),
            }
        }

        async fn delete_user_session(&self, _: String) -> Result<(), SessionRepositoryError> {
            todo!()
        }
    }

    #[tokio::test]
    async fn test_auth_session_found() {
        // Arrange
        let expected_session_token = "session_token".to_string();
        let expected_user_id: Uuid = Uuid::now_v7();
        let expected_username = "test".to_string();
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie = "csrf_cookie".to_string();
        let expected_user_session = UserSession {
            token: expected_session_token.clone(),
            user_id: expected_user_id,
            username: expected_username,
            csrf_token: expected_csrf_token.clone(),
            csrf_cookie: expected_csrf_cookie.clone(),
        };
        let mock_session_repository = Arc::new(MockSessionRepository {
            error: Mutex::new(None),
            return_value: expected_user_session.clone(),
        });
        let feature = UserAuthFeature::new(mock_session_repository);
        // Act
        let actual_user_session = feature.auth(expected_session_token).await.unwrap();
        // Assert
        assert_eq!(actual_user_session, expected_user_session);
    }

    #[tokio::test]
    async fn test_auth_session_not_found() {
        // Arrange
        let expected_session_token = "session_token".to_string();
        let expected_user_id: Uuid = Uuid::now_v7();
        let expected_username = "test".to_string();
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie = "csrf_cookie".to_string();
        let expected_user_session = UserSession {
            token: expected_session_token.clone(),
            user_id: expected_user_id,
            username: expected_username,
            csrf_token: expected_csrf_token.clone(),
            csrf_cookie: expected_csrf_cookie.clone(),
        };
        let mock_session_repository = Arc::new(MockSessionRepository {
            error: Mutex::new(Some(SessionRepositoryError::NotFound)),
            return_value: expected_user_session.clone(),
        });
        let feature = UserAuthFeature::new(mock_session_repository);
        // Act
        let actual_auth_result = feature.auth(expected_session_token).await;
        // Assert
        assert!(matches!(actual_auth_result, Err(SessionError::NotFound)));
    }
}
