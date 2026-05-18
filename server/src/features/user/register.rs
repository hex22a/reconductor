use std::sync::Arc;

use crate::features::user::error::UserError;
use crate::features::user::model::UserInsert;
use crate::features::user::repository::UserRepository;
use crate::infra::password::PasswordService;

pub(crate) trait RegisterFeature {
    fn register(
        &self,
        username: String,
        password: String,
    ) -> impl Future<Output = Result<(), UserError>> + Send;
}

#[derive(Clone)]
pub(crate) struct UserRegisterFeature<P: PasswordService, R: UserRepository> {
    password_service: Arc<P>,
    user_repository: Arc<R>,
}

impl<P: PasswordService, R: UserRepository> UserRegisterFeature<P, R> {
    pub(crate) fn new(password_service: Arc<P>, user_repository: Arc<R>) -> Self {
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
    async fn register(&self, username: String, password: String) -> Result<(), UserError> {
        let password_hash = self.password_service.hash_password(&password)?;
        self.user_repository
            .add_user(UserInsert {
                username,
                password_hash,
            })
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::features::user::model::UserEntity;
    use crate::features::user::model::UserInsert;
    use crate::features::user::register::RegisterFeature;
    use crate::features::user::register::UserRegisterFeature;
    use crate::features::user::repository::UserRepository;
    use crate::infra::password::PasswordService;
    use crate::infra::password::PasswordServiceError;

    struct MockUserRepository;
    struct MockPasswordService;
    impl UserRepository for MockUserRepository {
        async fn add_user(&self, _: UserInsert) -> Result<(), sqlx::Error> {
            Ok(())
        }

        async fn get_user_by_username(&self, _: &str) -> Result<UserEntity, sqlx::Error> {
            todo!()
        }
    }
    impl PasswordService for MockPasswordService {
        fn hash_password(&self, _: &str) -> Result<String, PasswordServiceError> {
            Ok("hashed_password".to_string())
        }

        fn verify_password(&self, _: &str, _: &str) -> Result<bool, PasswordServiceError> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn test_register_feature() {
        // Arrange
        let expected_username = "test".to_string();
        let expected_password = "password".to_string();
        let mock_password_service = Arc::new(MockPasswordService);
        let mock_user_repository = Arc::new(MockUserRepository);
        let feature = UserRegisterFeature::new(mock_password_service, mock_user_repository);
        // Act
        let actual_result = feature
            .register(expected_username, expected_password)
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_result, ());
    }
}
