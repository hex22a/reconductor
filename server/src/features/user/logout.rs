use std::sync::Arc;

use crate::features::{session::repository::SessionRepository, user::error::UserError};

pub trait LogoutFeature {
    fn logout(&self, session_id: &str) -> impl Future<Output = Result<(), UserError>> + Send;
}

#[derive(Clone)]
pub struct UserLogoutFeature<R: SessionRepository> {
    session_repository: Arc<R>,
}

impl<R: SessionRepository> UserLogoutFeature<R> {
    pub fn new(session_repository: Arc<R>) -> Self {
        Self { session_repository }
    }
}

impl<R> LogoutFeature for UserLogoutFeature<R>
where
    R: SessionRepository + Send + Sync,
{
    async fn logout(&self, session_id: &str) -> Result<(), UserError> {
        self.session_repository
            .delete_user_session(session_id)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::features::session::{model::UserSession, repository::SessionRepositoryError};

    use super::*;

    struct MockSessionRepository;
    impl SessionRepository for MockSessionRepository {
        async fn create_user_session(&self, _: UserSession) -> Result<(), SessionRepositoryError> {
            todo!()
        }

        async fn get_user_session(&self, _: &str) -> Result<UserSession, SessionRepositoryError> {
            todo!()
        }

        async fn delete_user_session(&self, _: &str) -> Result<(), SessionRepositoryError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_logout() {
        // Arrange
        let expected_session_id = "session_id";
        let mock_session_repository = MockSessionRepository;
        let feature = UserLogoutFeature::new(Arc::new(mock_session_repository));
        // Act
        let actual_logout_result = feature.logout(expected_session_id).await.unwrap();
        // Assert
        assert_eq!(actual_logout_result, ());
    }
}
