use std::sync::Arc;

use crate::{
    constants::USER_SESSION_TTL_SECONDS,
    features::{
        csrf::repository::CsrfRepository,
        session::model::UserSession,
        user::{error::UserError, model::AuthSession, repository::UserRepository},
    },
    infra::{
        csrf::CsrfService, password::PasswordService, persistence::kv::session::SessionRepository,
        random::RngService,
    },
};

pub trait LoginFeature {
    fn login(
        &self,
        username: String,
        password: String,
    ) -> impl Future<Output = Result<AuthSession, UserError>> + Send;
}

#[derive(Clone)]
pub struct UserLoginFeature<
    UR: UserRepository,
    SR: SessionRepository,
    CR: CsrfRepository,
    CS: CsrfService,
    PS: PasswordService,
    R: RngService,
> {
    user_repository: Arc<UR>,
    session_repository: Arc<SR>,
    csrf_repository: Arc<CR>,
    csrf_service: Arc<CS>,
    password_service: Arc<PS>,
    rng_service: Arc<R>,
}

impl<UR, SR, CR, CS, PS, R> UserLoginFeature<UR, SR, CR, CS, PS, R>
where
    UR: UserRepository + Send + Sync,
    SR: SessionRepository + Send + Sync,
    CR: CsrfRepository + Send + Sync,
    CS: CsrfService + Send + Sync,
    PS: PasswordService + Send + Sync,
    R: RngService + Send + Sync,
{
    pub fn new(
        user_repository: Arc<UR>,
        session_repository: Arc<SR>,
        csrf_repository: Arc<CR>,
        csrf_service: Arc<CS>,
        password_service: Arc<PS>,
        rng_service: Arc<R>,
    ) -> Self {
        Self {
            password_service,
            user_repository,
            session_repository,
            csrf_repository,
            csrf_service,
            rng_service,
        }
    }
}

impl<UR, SR, CR, CS, PS, R> LoginFeature for UserLoginFeature<UR, SR, CR, CS, PS, R>
where
    UR: UserRepository + Send + Sync,
    SR: SessionRepository + Send + Sync,
    CR: CsrfRepository + Send + Sync,
    CS: CsrfService + Send + Sync,
    PS: PasswordService + Send + Sync,
    R: RngService + Send + Sync,
{
    async fn login(&self, username: String, password: String) -> Result<AuthSession, UserError> {
        let user = self.user_repository.get_user_by_username(&username).await?;
        let is_valid = self
            .password_service
            .verify_password(&password, &user.password_hash)?;
        if is_valid {
            let session_id = self.rng_service.generate_session_id()?;
            let (csrf_token, csrf_cookie) = self.csrf_service.generate(USER_SESSION_TTL_SECONDS)?;
            self.session_repository
                .create_user_session(UserSession {
                    token: session_id.clone(),
                    csrf_token: csrf_token.clone(),
                    csrf_cookie,
                    user_id: user.id,
                    username: user.username,
                })
                .await?;
            Ok(AuthSession {
                session_id,
                csrf_token,
            })
        } else {
            Err(UserError::PasswordMismatch)
        }
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
        features::{
            csrf::repository::CsrfRepositoryError,
            user::model::{UserEntity, UserInsert},
        },
        infra::{
            csrf::CsrfServiceError,
            password::{PasswordService, PasswordServiceError},
            persistence::kv::session::SessionRepositoryError,
            random::RngServiceError,
        },
    };

    struct MockPasswordService {
        error: Mutex<Option<PasswordServiceError>>,
        is_valid: bool,
    }
    struct MockCsrfRepository;
    struct MockUserRepository {
        error: Mutex<Option<sqlx::Error>>,
        return_value: UserEntity,
    }
    struct MockSessionRepository;
    struct MockCsrfService {
        return_value: (String, String),
    }
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
        async fn create_user_session(&self, _: UserSession) -> Result<(), SessionRepositoryError> {
            Ok(())
        }

        async fn get_user_session(&self, _: String) -> Result<UserSession, SessionRepositoryError> {
            todo!()
        }

        async fn delete_user_session(&self, _: String) -> Result<(), SessionRepositoryError> {
            todo!()
        }
    }
    impl CsrfRepository for MockCsrfRepository {
        async fn create_anonymous_csrf(&self, _: String) -> Result<(), CsrfRepositoryError> {
            todo!()
        }

        async fn verify_anonymous_csrf(&self, _: String) -> Result<bool, CsrfRepositoryError> {
            todo!()
        }

        async fn delete_anonymous_csrf(&self, _: String) -> Result<(), CsrfRepositoryError> {
            Ok(())
        }
    }
    impl CsrfService for MockCsrfService {
        fn generate(&self, _: u64) -> Result<(String, String), CsrfServiceError> {
            Ok(self.return_value.clone())
        }

        fn verify(&self, _: &str, _: &str) -> bool {
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
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie_value = "csrf_cookie".to_string();
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
        let expected_csrf_service_generated_value = (
            expected_csrf_token.clone(),
            expected_csrf_cookie_value.clone(),
        );
        let expected_auth_session = AuthSession {
            session_id: expected_session_cookie.clone(),
            csrf_token: expected_csrf_token.clone(),
        };
        let mock_password_service = Arc::new(MockPasswordService {
            error: Mutex::new(None),
            is_valid: true,
        });
        let mock_user_repository = Arc::new(MockUserRepository {
            error: Mutex::new(None),
            return_value: expected_user_entity,
        });
        let mock_session_repository = Arc::new(MockSessionRepository);
        let mock_csrf_repository = Arc::new(MockCsrfRepository);
        let mock_csrf_service = Arc::new(MockCsrfService {
            return_value: expected_csrf_service_generated_value,
        });
        let mock_rng_service = Arc::new(MockRngService {
            return_value: expected_session_cookie.clone(),
        });
        let feature = UserLoginFeature::new(
            mock_user_repository,
            mock_session_repository,
            mock_csrf_repository,
            mock_csrf_service,
            mock_password_service,
            mock_rng_service,
        );
        // Act
        let actual_auth_session = feature
            .login(expected_username, expected_password)
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_auth_session, expected_auth_session);
    }

    #[tokio::test]
    async fn test_login_password_does_not_match() {
        // Arrange
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie_value = "csrf_cookie".to_string();
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
        let expected_csrf_service_generated_value =
            (expected_csrf_token, expected_csrf_cookie_value);
        let mock_password_service = Arc::new(MockPasswordService {
            error: Mutex::new(None),
            is_valid: false,
        });
        let mock_user_repository = Arc::new(MockUserRepository {
            error: Mutex::new(None),
            return_value: expected_user_entity,
        });
        let mock_session_repository = Arc::new(MockSessionRepository);
        let mock_csrf_repository = Arc::new(MockCsrfRepository);
        let mock_csrf_service = Arc::new(MockCsrfService {
            return_value: expected_csrf_service_generated_value,
        });
        let mock_rng_service = Arc::new(MockRngService {
            return_value: expected_session_cookie.clone(),
        });
        let feature = UserLoginFeature::new(
            mock_user_repository,
            mock_session_repository,
            mock_csrf_repository,
            mock_csrf_service,
            mock_password_service,
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
