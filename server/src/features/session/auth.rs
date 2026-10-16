use std::{pin::Pin, sync::Arc};

#[cfg(test)]
use mockall::automock;

use crate::{
    domain::types::SessionToken,
    features::session::{error::SessionError, model::UserSession, repository::SessionRepository},
};

#[cfg_attr(test, automock)]
pub trait AuthFeature {
    fn auth<'a>(
        &'a self,
        session_id: SessionToken,
    ) -> Pin<Box<dyn Future<Output = Result<UserSession, SessionError>> + Send + 'a>>;
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
    fn auth<'a>(
        &'a self,
        session_id: SessionToken,
    ) -> Pin<Box<dyn Future<Output = Result<UserSession, SessionError>> + Send + 'a>> {
        Box::pin(async move {
            Ok(self
                .session_repository
                .get_user_session(&session_id)
                .await?)
        })
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    use crate::{
        domain::types::{CsrfCookie, CsrfToken, Username},
        features::session::{
            model::UserSession,
            repository::{MockSessionRepository, SessionRepositoryError},
        },
    };

    #[tokio::test]
    async fn test_auth_session_found() {
        // Arrange
        let expected_session_token = SessionToken::new("session_token");
        let expected_user_id: Uuid = Uuid::now_v7();
        let expected_username = Username::new("test");
        let expected_csrf_token = CsrfToken::new("csrf_token");
        let expected_csrf_cookie = CsrfCookie::new("csrf_cookie");
        let expected_user_session = UserSession {
            token: expected_session_token.clone(),
            user_id: expected_user_id,
            username: expected_username,
            csrf_token: expected_csrf_token.clone(),
            csrf_cookie: expected_csrf_cookie.clone(),
        };

        let mut mock_session_repository = MockSessionRepository::new();
        let user_session = expected_user_session.clone();
        mock_session_repository
            .expect_get_user_session()
            .returning(move |_| {
                let user_session = user_session.clone();
                Box::pin(async { Ok(user_session) })
            });

        let feature = UserAuthFeature::new(Arc::new(mock_session_repository));

        // Act
        let actual_user_session = feature.auth(expected_session_token).await.unwrap();

        // Assert
        assert_eq!(actual_user_session, expected_user_session);
    }

    #[tokio::test]
    async fn test_auth_session_not_found() {
        // Arrange
        let expected_session_token = SessionToken::new("session_token");

        let mut mock_session_repository = MockSessionRepository::new();
        mock_session_repository
            .expect_get_user_session()
            .returning(|_| Box::pin(async { Err(SessionRepositoryError::NotFound) }));

        let feature = UserAuthFeature::new(Arc::new(mock_session_repository));

        // Act
        let actual_auth_result = feature.auth(expected_session_token).await;
        // Assert
        assert!(matches!(actual_auth_result, Err(SessionError::NotFound)));
    }
}
