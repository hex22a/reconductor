use std::sync::Arc;
use subtle::ConstantTimeEq;

use crate::infra::{csrf::CsrfService, persistence::kv::csrf::CsrfRepository};

pub trait VerifyCsrfFeature {
    fn verify_anonymous(
        &self,
        csrf_cookie: String,
        csrf_token: String,
    ) -> impl Future<Output = bool> + Send;
    fn verify_authorized(
        &self,
        csrf_cookie: String,
        header_csrf_token: String,
        session_csrf_token: String,
    ) -> impl Future<Output = bool> + Send;
}

pub struct StatefulCsrfVerifier<C: CsrfService, R: CsrfRepository> {
    csrf_service: Arc<C>,
    csrf_repository: Arc<R>,
}

impl<C: CsrfService, R: CsrfRepository> StatefulCsrfVerifier<C, R> {
    pub fn new(csrf_service: Arc<C>, csrf_repository: Arc<R>) -> Self {
        Self {
            csrf_service,
            csrf_repository,
        }
    }
}

impl<C, R> VerifyCsrfFeature for StatefulCsrfVerifier<C, R>
where
    C: CsrfService + Send + Sync,
    R: CsrfRepository + Send + Sync,
{
    async fn verify_anonymous(&self, csrf_cookie: String, csrf_token: String) -> bool {
        self.csrf_service.verify(&csrf_token, &csrf_cookie)
            && self
                .csrf_repository
                .verify_anonymous_csrf(csrf_token)
                .await
                .unwrap_or(false)
    }

    async fn verify_authorized(
        &self,
        csrf_cookie: String,
        header_csrf_token: String,
        session_csrf_token: String,
    ) -> bool {
        self.csrf_service.verify(&header_csrf_token, &csrf_cookie)
            && header_csrf_token
                .as_bytes()
                .ct_eq(session_csrf_token.as_bytes())
                .into()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use crate::infra::{csrf::CsrfServiceError, persistence::kv::csrf::CsrfRepositoryError};

    use super::*;

    struct MockCsrfRepository {
        error: Mutex<Option<CsrfRepositoryError>>,
        return_value: bool,
    }
    struct MockCsrfService {
        return_value: bool,
    }
    impl CsrfRepository for MockCsrfRepository {
        async fn create_anonymous_csrf(&self, _: String) -> Result<(), CsrfRepositoryError> {
            todo!()
        }

        async fn verify_anonymous_csrf(&self, _: String) -> Result<bool, CsrfRepositoryError> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(self.return_value.clone()),
            }
        }

        async fn delete_anonymous_csrf(&self, _: String) -> Result<(), CsrfRepositoryError> {
            todo!()
        }
    }
    impl CsrfService for MockCsrfService {
        fn generate(&self, _: u64) -> Result<(String, String), CsrfServiceError> {
            todo!()
        }

        fn verify(&self, _: &str, _: &str) -> bool {
            self.return_value.clone()
        }
    }

    #[tokio::test]
    async fn test_verify_anonymous_valid() {
        // Arrange
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie = "csrf_cookie".to_string();
        let mock_csrf_repository = Arc::new(MockCsrfRepository {
            error: Mutex::new(None),
            return_value: true,
        });
        let mock_csrf_service = Arc::new(MockCsrfService { return_value: true });
        let strategy = StatefulCsrfVerifier::new(mock_csrf_service, mock_csrf_repository);
        // Act
        let actual_is_valid = strategy
            .verify_anonymous(expected_csrf_cookie, expected_csrf_token)
            .await;
        // Assert
        assert_eq!(actual_is_valid, true);
    }

    #[tokio::test]
    async fn test_verify_anonymous_invalid_state() {
        // Arrange
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie = "csrf_cookie".to_string();
        let mock_csrf_repository = Arc::new(MockCsrfRepository {
            error: Mutex::new(None),
            return_value: false,
        });
        let mock_csrf_service = Arc::new(MockCsrfService { return_value: true });
        let strategy = StatefulCsrfVerifier::new(mock_csrf_service, mock_csrf_repository);
        // Act
        let actual_is_valid = strategy
            .verify_anonymous(expected_csrf_cookie, expected_csrf_token)
            .await;
        // Assert
        assert_eq!(actual_is_valid, false);
    }

    #[tokio::test]
    async fn test_verify_anonymous_storage_error() {
        // Arrange
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie = "csrf_cookie".to_string();
        let mock_csrf_repository = Arc::new(MockCsrfRepository {
            error: Mutex::new(Some(CsrfRepositoryError::StorageError(
                fred::error::Error::new_canceled(),
            ))),
            return_value: false,
        });
        let mock_csrf_service = Arc::new(MockCsrfService { return_value: true });
        let strategy = StatefulCsrfVerifier::new(mock_csrf_service, mock_csrf_repository);
        // Act
        let actual_is_valid = strategy
            .verify_anonymous(expected_csrf_cookie, expected_csrf_token)
            .await;
        // Assert
        assert_eq!(actual_is_valid, false);
    }

    #[tokio::test]
    async fn test_verify_anonymous_invalid() {
        // Arrange
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie = "csrf_cookie".to_string();
        let mock_csrf_repository = Arc::new(MockCsrfRepository {
            error: Mutex::new(None),
            return_value: false,
        });
        let mock_csrf_service = Arc::new(MockCsrfService {
            return_value: false,
        });
        let strategy = StatefulCsrfVerifier::new(mock_csrf_service, mock_csrf_repository);
        // Act
        let actual_is_valid = strategy
            .verify_anonymous(expected_csrf_cookie, expected_csrf_token)
            .await;
        // Assert
        assert_eq!(actual_is_valid, false);
    }

    #[tokio::test]
    async fn test_verify_authorized_valid() {
        // Arrange
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie = "csrf_cookie".to_string();
        let expected_session_csrf_token = "csrf_token".to_string();
        let mock_csrf_repository = Arc::new(MockCsrfRepository {
            error: Mutex::new(None),
            return_value: true,
        });
        let mock_csrf_service = Arc::new(MockCsrfService { return_value: true });
        let strategy = StatefulCsrfVerifier::new(mock_csrf_service, mock_csrf_repository);
        // Act
        let actual_is_valid = strategy
            .verify_authorized(
                expected_csrf_cookie,
                expected_csrf_token,
                expected_session_csrf_token,
            )
            .await;
        // Assert
        assert_eq!(actual_is_valid, true);
    }

    #[tokio::test]
    async fn test_verify_authorized_invalid_state() {
        // Arrange
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie = "csrf_cookie".to_string();
        let expected_session_csrf_token = "not_equal".to_string();
        let mock_csrf_repository = Arc::new(MockCsrfRepository {
            error: Mutex::new(None),
            return_value: true,
        });
        let mock_csrf_service = Arc::new(MockCsrfService { return_value: true });
        let strategy = StatefulCsrfVerifier::new(mock_csrf_service, mock_csrf_repository);
        // Act
        let actual_is_valid = strategy
            .verify_authorized(
                expected_csrf_cookie,
                expected_csrf_token,
                expected_session_csrf_token,
            )
            .await;
        // Assert
        assert_eq!(actual_is_valid, false);
    }

    #[tokio::test]
    async fn test_verify_authorized_invalid() {
        // Arrange
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie = "csrf_cookie".to_string();
        let expected_session_csrf_token = "csrf_token".to_string();
        let mock_csrf_repository = Arc::new(MockCsrfRepository {
            error: Mutex::new(None),
            return_value: true,
        });
        let mock_csrf_service = Arc::new(MockCsrfService {
            return_value: false,
        });
        let strategy = StatefulCsrfVerifier::new(mock_csrf_service, mock_csrf_repository);
        // Act
        let actual_is_valid = strategy
            .verify_authorized(
                expected_csrf_cookie,
                expected_csrf_token,
                expected_session_csrf_token,
            )
            .await;
        // Assert
        assert_eq!(actual_is_valid, false);
    }
}
