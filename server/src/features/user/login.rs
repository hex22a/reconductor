use crate::{
    features::user::error::UserError,
    infra::{
        password::PasswordService,
        persistence::{db::user::UserRepository, kv::session::SessionRepository},
        random::RngService,
    },
};

pub trait LoginFeature {
    fn login(
        &self,
        username: String,
        password: String,
    ) -> impl Future<Output = Result<String, UserError>> + Send;
}

#[derive(Clone)]
pub struct UserLoginFeature<
    P: PasswordService,
    R: UserRepository,
    S: SessionRepository,
    N: RngService,
> {
    password_service: P,
    user_repository: R,
    session_repository: S,
    rng_service: N,
}

impl<P, R, S, N> UserLoginFeature<P, R, S, N>
where
    P: PasswordService + Send + Sync,
    R: UserRepository + Send + Sync,
    S: SessionRepository + Send + Sync,
    N: RngService + Send + Sync,
{
    pub fn new(
        password_service: P,
        user_repository: R,
        session_repository: S,
        rng_service: N,
    ) -> Self {
        Self {
            password_service,
            user_repository,
            session_repository,
            rng_service,
        }
    }
}

impl<P, R, S, N> LoginFeature for UserLoginFeature<P, R, S, N>
where
    P: PasswordService + Send + Sync,
    R: UserRepository + Send + Sync,
    S: SessionRepository + Send + Sync,
    N: RngService + Send + Sync,
{
    async fn login(&self, username: String, password: String) -> Result<String, UserError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;
    use uuid::Uuid;

    use super::*;
    use std::sync::Mutex;

    use crate::{
        constants::NONCE_SIZE_BYTES,
        infra::{
            password::{PasswordService, PasswordServiceError},
            persistence::{
                db::user::{UserEntity, UserInsert},
                kv::session::{SessionError, UserSession},
            },
            random::RngServiceError,
        },
    };

    struct MockPasswordService {
        error: Mutex<Option<PasswordServiceError>>,
        is_valid: bool,
    }
    struct MockUserRepository {
        error: Mutex<Option<sqlx::Error>>,
        return_value: UserEntity,
    }
    struct MockSessionRepository;
    struct MockRngService {
        return_value: String,
    }
    impl PasswordService for MockPasswordService {
        fn hash_password(&self, _: &str) -> Result<String, PasswordServiceError> {
            todo!()
        }

        fn verify_password(&self, _: &str, _: &str) -> Result<bool, PasswordServiceError> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(self.is_valid.clone()),
            }
        }
    }
    impl UserRepository for MockUserRepository {
        async fn add_user(&self, _: UserInsert) -> Result<(), sqlx::Error> {
            todo!()
        }

        async fn get_user_by_username(&self, _: &str) -> Result<UserEntity, sqlx::Error> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(self.return_value.clone()),
            }
        }
    }
    impl SessionRepository for MockSessionRepository {
        async fn create_user_session(&self, _: UserSession) -> Result<(), SessionError> {
            Ok(())
        }

        async fn get_user_session(&self, _: String) -> Result<UserSession, SessionError> {
            todo!()
        }

        async fn delete_user_session(&self, _: String) -> Result<(), SessionError> {
            todo!()
        }
    }
    impl RngService for MockRngService {
        fn generate_nonce(&self) -> Result<[u8; NONCE_SIZE_BYTES], RngServiceError> {
            todo!()
        }

        fn generate_session_id(&self) -> Result<String, RngServiceError> {
            Ok(self.return_value.clone())
        }
    }

    #[tokio::test]
    async fn test_login_password_matches() {
        // Arrange
        let expected_session_cookie = "session_cookie".to_string();
        let expected_user_id = Uuid::now_v7();
        let expected_username = "test".to_string();
        let expected_password = "password".to_string();
        let expected_password_hash = "password_hash".to_string();
        let expected_password_version: i16 = 1;
        let expected_created_at = datetime!(2019-01-01 0:00);
        let expected_updated_at = datetime!(2019-01-01 0:00);
        let expected_last_login_at = datetime!(2019-01-01 0:00);
        let expected_is_active = true;
        let expected_user_entity = UserEntity {
            id: expected_user_id,
            username: expected_username.clone(),
            password_hash: expected_password_hash,
            password_version: expected_password_version,
            created_at: expected_created_at,
            updated_at: expected_updated_at,
            last_login_at: expected_last_login_at,
            is_active: expected_is_active,
        };
        let mock_password_service = MockPasswordService {
            error: Mutex::new(None),
            is_valid: true,
        };
        let mock_user_repository = MockUserRepository {
            error: Mutex::new(None),
            return_value: expected_user_entity,
        };
        let mock_session_repository = MockSessionRepository;
        let mock_rng_service = MockRngService {
            return_value: expected_session_cookie.clone(),
        };
        let feature = UserLoginFeature::new(
            mock_password_service,
            mock_user_repository,
            mock_session_repository,
            mock_rng_service,
        );
        // Act
        let actual_session_cookie = feature
            .login(expected_username, expected_password)
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_session_cookie, expected_session_cookie);
    }

    #[tokio::test]
    async fn test_login_password_does_not_match() {
        // Arrange
        let expected_user_id = Uuid::now_v7();
        let expected_session_cookie = "session_cookie".to_string();
        let expected_username = "test".to_string();
        let expected_password = "password".to_string();
        let expected_password_hash = "password_hash".to_string();
        let expected_password_version: i16 = 1;
        let expected_created_at = datetime!(2019-01-01 0:00);
        let expected_updated_at = datetime!(2019-01-01 0:00);
        let expected_last_login_at = datetime!(2019-01-01 0:00);
        let expected_is_active = true;
        let expected_user_entity = UserEntity {
            id: expected_user_id,
            username: expected_username.clone(),
            password_hash: expected_password_hash,
            password_version: expected_password_version,
            created_at: expected_created_at,
            updated_at: expected_updated_at,
            last_login_at: expected_last_login_at,
            is_active: expected_is_active,
        };
        let mock_password_service = MockPasswordService {
            error: Mutex::new(None),
            is_valid: false,
        };
        let mock_user_repository = MockUserRepository {
            error: Mutex::new(None),
            return_value: expected_user_entity,
        };
        let mock_session_repository = MockSessionRepository;
        let mock_rng_service = MockRngService {
            return_value: expected_session_cookie.clone(),
        };
        let feature = UserLoginFeature::new(
            mock_password_service,
            mock_user_repository,
            mock_session_repository,
            mock_rng_service,
        );
        // Act
        let actual_login_result = feature.login(expected_username, expected_password).await;
        // Assert
        assert!(matches!(
            actual_login_result,
            Err(UserError::PasswordMismatch)
        ));
    }
}
