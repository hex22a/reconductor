use std::{pin::Pin, sync::Arc};

#[cfg(test)]
use mockall::automock;

use crate::{
    constants::ANONYMOUS_CSRF_TTL_SECONDS,
    features::{
        csrf::{error::CsrfError, model::CsrfTokenPair, repository::CsrfRepository},
        session::repository::{SessionRepository, SessionRepositoryError},
    },
    infra::csrf::CsrfService,
};

#[cfg_attr(test, automock)]
pub trait TokenFeature {
    fn get_token<'a>(
        &'a self,
        session_token: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<CsrfTokenPair, CsrfError>> + Send + 'a>>;
}

#[derive(Clone)]
pub struct CsrfTokenFeature<R: SessionRepository, C: CsrfRepository, S: CsrfService> {
    session_repository: Arc<R>,
    csrf_repository: Arc<C>,
    csrf_service: Arc<S>,
}

impl<R: SessionRepository, C: CsrfRepository, S: CsrfService> CsrfTokenFeature<R, C, S> {
    pub fn new(session_repository: Arc<R>, csrf_repository: Arc<C>, csrf_service: Arc<S>) -> Self {
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
    fn get_token<'a>(
        &'a self,
        session_cookie: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<CsrfTokenPair, CsrfError>> + Send + 'a>> {
        Box::pin(async move {
            match session_cookie {
                Some(token) => {
                    let user_session_result =
                        self.session_repository.get_user_session(&token).await;
                    match user_session_result {
                        Ok(user_session) => Ok(CsrfTokenPair {
                            token: user_session.csrf_token,
                            cookie_value: user_session.csrf_cookie,
                        }),
                        Err(error) => match error {
                            SessionRepositoryError::NotFound => {
                                let (token, cookie_value) =
                                    self.csrf_service.generate(ANONYMOUS_CSRF_TTL_SECONDS)?;
                                self.csrf_repository.create_anonymous_csrf(&token).await?;
                                Ok(CsrfTokenPair {
                                    token,
                                    cookie_value,
                                })
                            }
                            SessionRepositoryError::ParseError => Err(CsrfError::StorageError(
                                "error parsing user session".to_string(),
                            )),
                            SessionRepositoryError::StorageError(e) => {
                                Err(CsrfError::StorageError(e.to_string()))
                            }
                        },
                    }
                }
                None => {
                    let (token, cookie_value) =
                        self.csrf_service.generate(ANONYMOUS_CSRF_TTL_SECONDS)?;
                    self.csrf_repository.create_anonymous_csrf(&token).await?;
                    Ok(CsrfTokenPair {
                        token,
                        cookie_value,
                    })
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::{
        features::{
            csrf::repository::MockCsrfRepository,
            session::{model::UserSession, repository::MockSessionRepository},
        },
        infra::csrf::MockCsrfService,
    };

    #[tokio::test]
    async fn test_get_token_from_session() {
        // Arrange
        let expected_session_token = String::from("session_token");
        let expected_user_id: Uuid = Uuid::now_v7();
        let expected_username = String::from("test");
        let expected_csrf_token = String::from("csrf_token");
        let expected_csrf_cookie = String::from("csrf_cookie");
        let expected_csrf_token_pair = CsrfTokenPair {
            token: expected_csrf_token.clone(),
            cookie_value: expected_csrf_cookie.clone(),
        };
        let expected_user_session = UserSession {
            token: expected_session_token.clone(),
            user_id: expected_user_id,
            username: expected_username,
            csrf_token: expected_csrf_token.clone(),
            csrf_cookie: expected_csrf_cookie.clone(),
        };

        let mut mock_session_repository = MockSessionRepository::new();
        mock_session_repository
            .expect_get_user_session()
            .returning(move |_| {
                let user_session = expected_user_session.clone();
                Box::pin(async { Ok(user_session) })
            });

        let mut mock_csrf_repository = MockCsrfRepository::new();
        mock_csrf_repository
            .expect_create_anonymous_csrf()
            .returning(|_| Box::pin(async { Ok(()) }));

        let mut mock_csrf_service = MockCsrfService::new();
        mock_csrf_service
            .expect_generate()
            .return_const(Ok((expected_csrf_token, expected_csrf_cookie)));

        let feature = CsrfTokenFeature::new(
            Arc::new(mock_session_repository),
            Arc::new(mock_csrf_repository),
            Arc::new(mock_csrf_service),
        );

        // Act
        let actual_csrf_token_pair = feature
            .get_token(Some(expected_session_token))
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_csrf_token_pair, expected_csrf_token_pair);
    }

    #[tokio::test]
    async fn test_get_token_anonymous_token() {
        // Arrange
        let expected_session_token = "session_token".to_string();
        let expected_anonymous_csrf_token = "anonymous_csrf".to_string();
        let expected_csrf_cookie = "csrf_cookie".to_string();
        let expected_csrf_token_pair = CsrfTokenPair {
            token: expected_anonymous_csrf_token.clone(),
            cookie_value: expected_csrf_cookie.clone(),
        };

        let mut mock_session_repository = MockSessionRepository::new();
        mock_session_repository
            .expect_get_user_session()
            .returning(|_| Box::pin(async { Err(SessionRepositoryError::NotFound) }));

        let mut mock_csrf_repository = MockCsrfRepository::new();
        mock_csrf_repository
            .expect_create_anonymous_csrf()
            .returning(|_| Box::pin(async { Ok(()) }));

        let mut mock_csrf_service = MockCsrfService::new();
        mock_csrf_service
            .expect_generate()
            .return_const(Ok((expected_anonymous_csrf_token, expected_csrf_cookie)));

        let feature = CsrfTokenFeature::new(
            Arc::new(mock_session_repository),
            Arc::new(mock_csrf_repository),
            Arc::new(mock_csrf_service),
        );

        // Act
        let actual_csrf_token_pair = feature
            .get_token(Some(expected_session_token))
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_csrf_token_pair, expected_csrf_token_pair);
    }

    #[tokio::test]
    async fn test_get_token_no_cookie_passed() {
        // Arrange
        let expected_anonymous_csrf_token = "anonymous_csrf".to_string();
        let expected_csrf_cookie = "csrf_cookie".to_string();
        let expected_csrf_token_pair = CsrfTokenPair {
            token: expected_anonymous_csrf_token.clone(),
            cookie_value: expected_csrf_cookie.clone(),
        };

        let mut mock_session_repository = MockSessionRepository::new();
        mock_session_repository
            .expect_get_user_session()
            .returning(|_| Box::pin(async { Err(SessionRepositoryError::NotFound) }));

        let mut mock_csrf_repository = MockCsrfRepository::new();
        mock_csrf_repository
            .expect_create_anonymous_csrf()
            .returning(|_| Box::pin(async { Ok(()) }));

        let mut mock_csrf_service = MockCsrfService::new();
        mock_csrf_service
            .expect_generate()
            .return_const(Ok((expected_anonymous_csrf_token, expected_csrf_cookie)));

        let feature = CsrfTokenFeature::new(
            Arc::new(mock_session_repository),
            Arc::new(mock_csrf_repository),
            Arc::new(mock_csrf_service),
        );

        // Act
        let actual_csrf_token_pair = feature.get_token(None).await.unwrap();

        // Assert
        assert_eq!(actual_csrf_token_pair, expected_csrf_token_pair);
    }
}
