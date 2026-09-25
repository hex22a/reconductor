use std::{pin::Pin, sync::Arc};

use crate::features::{session::repository::SessionRepository, user::error::UserError};

pub trait LogoutFeature {
    fn logout<'a>(
        &'a self,
        session_id: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), UserError>> + Send + 'a>>;
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
    fn logout<'a>(
        &'a self,
        session_id: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), UserError>> + Send + 'a>> {
        Box::pin(async move {
            self.session_repository
                .delete_user_session(session_id)
                .await?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::features::session::repository::MockSessionRepository;

    use super::*;

    #[tokio::test]
    async fn test_logout() {
        // Arrange
        let expected_session_id = "session_id";
        let mut mock_session_repository = MockSessionRepository::new();
        mock_session_repository
            .expect_delete_user_session()
            .returning(|_| Box::pin(async { Ok(()) }));

        let feature = UserLogoutFeature::new(Arc::new(mock_session_repository));

        // Act
        let actual_logout_result = feature.logout(expected_session_id).await;

        // Assert
        assert!(actual_logout_result.is_ok());
    }
}
