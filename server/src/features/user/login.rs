use std::{pin::Pin, sync::Arc};

use crate::{
    constants::USER_SESSION_TTL_SECONDS,
    features::{
        csrf::repository::CsrfRepository,
        session::{model::UserSession, repository::SessionRepository},
        user::{error::UserError, model::AuthSession, repository::UserRepository},
    },
    infra::{csrf::CsrfService, password::PasswordService, random::RngService},
};

pub trait LoginFeature {
    fn login(
        &self,
        username: String,
        password: String,
        anonymos_csrf_token: String,
    ) -> Pin<Box<dyn Future<Output = Result<AuthSession, UserError>> + Send + '_>>;
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
    fn login(
        &self,
        username: String,
        password: String,
        anonymous_csrf_token: String,
    ) -> Pin<Box<dyn Future<Output = Result<AuthSession, UserError>> + Send + '_>> {
        Box::pin(async move {
            let user = self.user_repository.get_user_by_username(&username).await?;
            let is_valid = self
                .password_service
                .verify_password(&password, &user.password_hash)?;
            if is_valid {
                let session_id = self.rng_service.generate_session_id()?;
                let (csrf_token, csrf_cookie) =
                    self.csrf_service.generate(USER_SESSION_TTL_SECONDS)?;
                self.session_repository
                    .create_user_session(UserSession {
                        token: session_id.clone(),
                        csrf_token: csrf_token.clone(),
                        csrf_cookie: csrf_cookie.clone(),
                        user_id: user.id,
                        username: user.username,
                    })
                    .await?;
                self.csrf_repository
                    .delete_anonymous_csrf(&anonymous_csrf_token)
                    .await?;
                Ok(AuthSession {
                    session_id,
                    csrf_token,
                    csrf_cookie,
                })
            } else {
                Err(UserError::PasswordMismatch)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;
    use uuid::Uuid;

    use super::*;

    use crate::{
        features::{
            csrf::repository::MockCsrfRepository,
            session::repository::MockSessionRepository,
            user::{model::UserEntity, repository::MockUserRepository},
        },
        infra::{csrf::MockCsrfService, password::MockPasswordService, random::MockRngService},
    };

    #[tokio::test]
    async fn test_login_password_matches() {
        // Arrange
        let expected_anonymous_csrf = "anonymous_csrf".to_string();
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie_value = "csrf_cookie".to_string();
        let expected_session_cookie = "session_cookie".to_string();
        let expected_user_id = Uuid::now_v7();
        let expected_username = "test".to_string();
        let expected_password = "password".to_string();
        let expected_password_hash = "password_hash".to_string();
        let expected_password_version: i16 = 1;
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_updated_at = datetime!(2019-01-01 0:00 UTC);
        let expected_last_login_at = datetime!(2019-01-01 0:00 UTC);
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
            csrf_cookie: expected_csrf_cookie_value.clone(),
        };

        let mut mock_password_service = MockPasswordService::new();
        mock_password_service
            .expect_verify_password()
            .return_const(Ok(true));

        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_get_user_by_username()
            .returning(move |_| {
                let user = expected_user_entity.clone();
                Box::pin(async { Ok(user) })
            });

        let mut mock_session_repository = MockSessionRepository::new();
        mock_session_repository
            .expect_create_user_session()
            .returning(|_| Box::pin(async { Ok(()) }));

        let mut mock_csrf_repository = MockCsrfRepository::new();
        mock_csrf_repository
            .expect_delete_anonymous_csrf()
            .returning(|_| Box::pin(async { Ok(()) }));

        let mut mock_csrf_service = MockCsrfService::new();
        mock_csrf_service
            .expect_generate()
            .return_const(Ok(expected_csrf_service_generated_value));

        let mut mock_rng_service = MockRngService::new();
        mock_rng_service
            .expect_generate_session_id()
            .return_const(Ok(expected_session_cookie));

        let feature = UserLoginFeature::new(
            Arc::new(mock_user_repository),
            Arc::new(mock_session_repository),
            Arc::new(mock_csrf_repository),
            Arc::new(mock_csrf_service),
            Arc::new(mock_password_service),
            Arc::new(mock_rng_service),
        );

        // Act
        let actual_auth_session = feature
            .login(
                expected_username,
                expected_password,
                expected_anonymous_csrf,
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_auth_session, expected_auth_session);
    }

    #[tokio::test]
    async fn test_login_password_does_not_match() {
        // Arrange
        let expected_anonymous_csrf = "anonymous_csrf".to_string();
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie_value = "csrf_cookie".to_string();
        let expected_user_id = Uuid::now_v7();
        let expected_session_cookie = "session_cookie".to_string();
        let expected_username = "test".to_string();
        let expected_password = "password".to_string();
        let expected_password_hash = "password_hash".to_string();
        let expected_password_version: i16 = 1;
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_updated_at = datetime!(2019-01-01 0:00 UTC);
        let expected_last_login_at = datetime!(2019-01-01 0:00 UTC);
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

        let mut mock_password_service = MockPasswordService::new();
        mock_password_service
            .expect_verify_password()
            .return_const(Ok(false));

        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_get_user_by_username()
            .returning(move |_| {
                let user = expected_user_entity.clone();
                Box::pin(async { Ok(user) })
            });

        let mut mock_session_repository = MockSessionRepository::new();
        mock_session_repository
            .expect_create_user_session()
            .returning(|_| Box::pin(async { Ok(()) }));

        let mut mock_csrf_repository = MockCsrfRepository::new();
        mock_csrf_repository
            .expect_delete_anonymous_csrf()
            .returning(|_| Box::pin(async { Ok(()) }));

        let mut mock_csrf_service = MockCsrfService::new();
        mock_csrf_service
            .expect_generate()
            .return_const(Ok(expected_csrf_service_generated_value));

        let mut mock_rng_service = MockRngService::new();
        mock_rng_service
            .expect_generate_session_id()
            .return_const(Ok(expected_session_cookie));

        let feature = UserLoginFeature::new(
            Arc::new(mock_user_repository),
            Arc::new(mock_session_repository),
            Arc::new(mock_csrf_repository),
            Arc::new(mock_csrf_service),
            Arc::new(mock_password_service),
            Arc::new(mock_rng_service),
        );

        // Act
        let actual_login_result = feature
            .login(
                expected_username,
                expected_password,
                expected_anonymous_csrf,
            )
            .await;

        // Assert
        assert!(matches!(
            actual_login_result,
            Err(UserError::PasswordMismatch)
        ));
    }
}
