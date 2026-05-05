use crate::domain::error::ServerError;
use crate::infra::password::PasswordService;
use crate::persistence::db::user::{UserInsert, UserRepository};

pub trait RegisterFeature {
    async fn register(&self, username: &str, password: &str) -> Result<(), ServerError>;
}

struct UserRegisterFeature<P: PasswordService, R: UserRepository> {
    password_service: P,
    user_repository: R,
}

impl<P: PasswordService, R: UserRepository> UserRegisterFeature<P, R> {
    fn new(password_service: P, user_repository: R) -> Self {
        Self {
            password_service,
            user_repository,
        }
    }
}

impl<P: PasswordService, R: UserRepository> RegisterFeature for UserRegisterFeature<P, R> {
    async fn register(&self, username: &str, password: &str) -> Result<(), ServerError> {
        let password_hash = self.password_service.hash_password(password)?;
        self.user_repository
            .add_user(UserInsert {
                username: username.to_string(),
                password_hash,
            })
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::features::user::register::RegisterFeature;
    use crate::features::user::register::UserRegisterFeature;
    use crate::infra::password::PasswordService;
    use crate::infra::password::PasswordServiceError;
    use crate::persistence::db::user::UserEntity;
    use crate::persistence::db::user::UserInsert;
    use crate::persistence::db::user::UserRepository;

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
        let expected_username = "test";
        let expected_password = "password";
        let mock_password_service = MockPasswordService;
        let mock_user_repository = MockUserRepository;
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
