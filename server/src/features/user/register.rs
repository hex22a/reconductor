use std::pin::Pin;
use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

use crate::features::user::error::UserError;
use crate::features::user::model::UserInsert;
use crate::features::user::repository::UserRepository;
use crate::infra::password::PasswordService;

#[cfg_attr(test, automock)]
pub trait RegisterFeature {
    fn register<'a>(
        &'a self,
        username: String,
        password: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), UserError>> + Send + 'a>>;
}

#[derive(Clone)]
pub struct UserRegisterFeature<P: PasswordService, R: UserRepository> {
    password_service: Arc<P>,
    user_repository: Arc<R>,
}

impl<P: PasswordService, R: UserRepository> UserRegisterFeature<P, R> {
    pub fn new(password_service: Arc<P>, user_repository: Arc<R>) -> Self {
        Self {
            password_service,
            user_repository,
        }
    }
}

impl<P, R> RegisterFeature for UserRegisterFeature<P, R>
where
    P: PasswordService + Send + Sync,
    R: UserRepository + Send + Sync,
{
    fn register(
        &self,
        username: String,
        password: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), UserError>> + Send + '_>> {
        Box::pin(async move {
            let password_hash = self.password_service.hash_password(&password)?;
            self.user_repository
                .add_user(UserInsert {
                    username,
                    password_hash,
                })
                .await?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::features::user::register::RegisterFeature;
    use crate::features::user::register::UserRegisterFeature;
    use crate::features::user::repository::MockUserRepository;
    use crate::infra::password::MockPasswordService;

    #[tokio::test]
    async fn test_register_feature() {
        // Arrange
        let expected_username = "test".to_string();
        let expected_password = "password".to_string();
        let mut mock_password_service = MockPasswordService::new();
        mock_password_service
            .expect_hash_password()
            .return_const(Ok(String::from("hashed_password")));
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_add_user()
            .returning(|_| Box::pin(async { Ok(()) }));
        let feature = UserRegisterFeature::new(
            Arc::new(mock_password_service),
            Arc::new(mock_user_repository),
        );

        // Act
        let actual_result = feature.register(expected_username, expected_password).await;

        // Assert
        assert!(actual_result.is_ok());
    }
}
