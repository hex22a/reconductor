use crate::{
    constants::ANONYMOUS_CSRF_TTL_SECONDS,
    features::csrf::{error::CsrfError, model::CsrfTokenPair},
    infra::{
        csrf::CsrfService,
        persistence::kv::{
            csrf::CsrfRepository,
            session::{SessionError, SessionRepository},
        },
    },
};

pub trait TokenFeature {
    fn get_token(
        &self,
        session_token: Option<String>,
    ) -> impl Future<Output = Result<CsrfTokenPair, CsrfError>> + Send;
}

#[derive(Clone)]
pub struct CsrfTokenFeature<R: SessionRepository, C: CsrfRepository, S: CsrfService> {
    session_repository: R,
    csrf_repository: C,
    csrf_service: S,
}

impl<R: SessionRepository, C: CsrfRepository, S: CsrfService> CsrfTokenFeature<R, C, S> {
    pub fn new(session_repository: R, csrf_repository: C, csrf_service: S) -> Self {
        Self {
            session_repository,
            csrf_repository,
            csrf_service,
        }
    }
}

impl<R, C, S> TokenFeature for CsrfTokenFeature<R, C, S>
where
    R: SessionRepository + Send + Sync,
    C: CsrfRepository + Send + Sync,
    S: CsrfService + Send + Sync,
{
    async fn get_token(&self, session_cookie: Option<String>) -> Result<CsrfTokenPair, CsrfError> {
        match session_cookie {
            Some(token) => {
                let user_session_result = self.session_repository.get_user_session(token).await;
                match user_session_result {
                    Ok(user_session) => Ok(CsrfTokenPair {
                        token: user_session.csrf_token,
                        cookie_value: None,
                    }),
                    Err(error) => match error {
                        SessionError::NotFound => {
                            let (token, cookie_value) =
                                self.csrf_service.generate(ANONYMOUS_CSRF_TTL_SECONDS)?;
                            self.csrf_repository
                                .create_anonymous_csrf(token.clone())
                                .await?;
                            Ok(CsrfTokenPair {
                                token,
                                cookie_value: Some(cookie_value),
                            })
                        }
                        SessionError::ParseError => Err(CsrfError::StorageError(
                            "error parsing user session".to_string(),
                        )),
                        SessionError::StorageError(e) => {
                            Err(CsrfError::StorageError(e.to_string()))
                        }
                    },
                }
            }
            None => {
                let (token, cookie_value) =
                    self.csrf_service.generate(ANONYMOUS_CSRF_TTL_SECONDS)?;
                self.csrf_repository
                    .create_anonymous_csrf(token.clone())
                    .await?;
                Ok(CsrfTokenPair {
                    token,
                    cookie_value: Some(cookie_value),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use uuid::Uuid;

    use super::*;
    use crate::infra::{
        csrf::{CsrfService, CsrfServiceError},
        persistence::kv::{csrf::CsrfRepositoryError, session::UserSession},
    };

    struct MockSessionRepository {
        error: Mutex<Option<SessionError>>,
        return_value: UserSession,
    }
    struct MockCsrfRepository;
    struct MockCsrfService {
        error: Mutex<Option<CsrfServiceError>>,
        return_value: (String, String),
    }
    impl SessionRepository for MockSessionRepository {
        async fn create_user_session(&self, _: UserSession) -> Result<(), SessionError> {
            todo!()
        }

        async fn get_user_session(&self, _: String) -> Result<UserSession, SessionError> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(self.return_value.clone()),
            }
        }

        async fn delete_user_session(&self, _: String) -> Result<(), SessionError> {
            todo!()
        }
    }
    impl CsrfRepository for MockCsrfRepository {
        async fn create_anonymous_csrf(&self, _: String) -> Result<(), CsrfRepositoryError> {
            Ok(())
        }

        async fn verify_anonymous_csrf(&self, _: String) -> Result<bool, CsrfRepositoryError> {
            todo!()
        }

        async fn delete_anonymous_csrf(&self, _: String) -> Result<(), CsrfRepositoryError> {
            todo!()
        }
    }
    impl CsrfService for MockCsrfService {
        fn generate(&self, _: u64) -> Result<(String, String), CsrfServiceError> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(self.return_value.clone()),
            }
        }

        fn verify(&self, _: &str, _: &str) -> bool {
            todo!()
        }
    }

    #[tokio::test]
    async fn test_get_token_from_session() {
        // Arrange
        let expected_session_token = "session_token".to_string();
        let expected_user_id: Uuid = Uuid::now_v7();
        let expected_username = "test".to_string();
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie = "csrf_cookie".to_string();
        let expected_csrf_token_pair = CsrfTokenPair {
            token: expected_csrf_token.clone(),
            cookie_value: None,
        };
        let expected_user_session = UserSession {
            token: expected_session_token.clone(),
            user_id: expected_user_id,
            username: expected_username,
            csrf_token: expected_csrf_token.clone(),
        };
        let mock_session_repository = MockSessionRepository {
            error: Mutex::new(None),
            return_value: expected_user_session,
        };
        let mock_csrf_repository = MockCsrfRepository;
        let mock_csrf_service = MockCsrfService {
            error: Mutex::new(None),
            return_value: (expected_csrf_token.clone(), expected_csrf_cookie),
        };
        let feature = CsrfTokenFeature::new(
            mock_session_repository,
            mock_csrf_repository,
            mock_csrf_service,
        );
        // Act
        let actual_csrf_token_pair = feature
            .get_token(Some(expected_session_token.clone()))
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_csrf_token_pair, expected_csrf_token_pair);
    }

    #[tokio::test]
    async fn test_get_token_anonymous_token() {
        // Arrange
        let expected_session_token = "session_token".to_string();
        let expected_user_id: Uuid = Uuid::now_v7();
        let expected_username = "test".to_string();
        let expected_csrf_token = "csrf_token".to_string();
        let expected_anonymous_csrf_token = "anonymous_csrf".to_string();
        let expected_csrf_cookie = "csrf_cookie".to_string();
        let expected_csrf_token_pair = CsrfTokenPair {
            token: expected_anonymous_csrf_token.clone(),
            cookie_value: Some(expected_csrf_cookie.clone()),
        };
        let expected_user_session = UserSession {
            token: expected_session_token.clone(),
            user_id: expected_user_id,
            username: expected_username,
            csrf_token: expected_csrf_token.clone(),
        };
        let mock_session_repository = MockSessionRepository {
            error: Mutex::new(Some(SessionError::NotFound)),
            return_value: expected_user_session,
        };
        let mock_csrf_repository = MockCsrfRepository;
        let mock_csrf_service = MockCsrfService {
            error: Mutex::new(None),
            return_value: (expected_anonymous_csrf_token.clone(), expected_csrf_cookie),
        };
        let feature = CsrfTokenFeature::new(
            mock_session_repository,
            mock_csrf_repository,
            mock_csrf_service,
        );
        // Act
        let actual_csrf_token_pair = feature
            .get_token(Some(expected_session_token.clone()))
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_csrf_token_pair, expected_csrf_token_pair);
    }

    #[tokio::test]
    async fn test_get_token_no_cookie_passed() {
        // Arrange
        let expected_session_token = "session_token".to_string();
        let expected_user_id: Uuid = Uuid::now_v7();
        let expected_username = "test".to_string();
        let expected_csrf_token = "csrf_token".to_string();
        let expected_anonymous_csrf_token = "anonymous_csrf".to_string();
        let expected_csrf_cookie = "csrf_cookie".to_string();
        let expected_csrf_token_pair = CsrfTokenPair {
            token: expected_anonymous_csrf_token.clone(),
            cookie_value: Some(expected_csrf_cookie.clone()),
        };
        let expected_user_session = UserSession {
            token: expected_session_token.clone(),
            user_id: expected_user_id,
            username: expected_username,
            csrf_token: expected_csrf_token.clone(),
        };
        let mock_session_repository = MockSessionRepository {
            error: Mutex::new(Some(SessionError::NotFound)),
            return_value: expected_user_session,
        };
        let mock_csrf_repository = MockCsrfRepository;
        let mock_csrf_service = MockCsrfService {
            error: Mutex::new(None),
            return_value: (expected_anonymous_csrf_token.clone(), expected_csrf_cookie),
        };
        let feature = CsrfTokenFeature::new(
            mock_session_repository,
            mock_csrf_repository,
            mock_csrf_service,
        );
        // Act
        let actual_csrf_token_pair = feature.get_token(None).await.unwrap();
        // Assert
        assert_eq!(actual_csrf_token_pair, expected_csrf_token_pair);
    }
}
