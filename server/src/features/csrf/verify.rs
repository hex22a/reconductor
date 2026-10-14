use std::{pin::Pin, sync::Arc};
use subtle::ConstantTimeEq;

#[cfg(test)]
use mockall::automock;

use crate::{
    features::csrf::{
        model::{CsrfCookie, CsrfToken},
        repository::CsrfRepository,
    },
    infra::csrf::CsrfService,
};

#[cfg_attr(test, automock)]
pub trait VerifyCsrfFeature {
    fn verify_anonymous<'a>(
        &'a self,
        csrf_cookie: CsrfCookie,
        csrf_token: CsrfToken,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'a>>;
    fn verify_authorized<'a>(
        &'a self,
        csrf_cookie: CsrfCookie,
        header_csrf_token: CsrfToken,
        session_csrf_token: CsrfToken,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'a>>;
}

#[derive(Clone)]
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
    fn verify_anonymous(
        &self,
        csrf_cookie: CsrfCookie,
        csrf_token: CsrfToken,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + '_>> {
        Box::pin(async move {
            self.csrf_service.verify(&csrf_token, &csrf_cookie)
                && self
                    .csrf_repository
                    .verify_anonymous_csrf(&csrf_token)
                    .await
                    .unwrap_or(false)
        })
    }

    fn verify_authorized(
        &self,
        csrf_cookie: CsrfCookie,
        header_csrf_token: CsrfToken,
        session_csrf_token: CsrfToken,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + '_>> {
        Box::pin(async move {
            self.csrf_service.verify(&header_csrf_token, &csrf_cookie)
                && header_csrf_token
                    .as_ref()
                    .as_bytes()
                    .ct_eq(session_csrf_token.as_ref().as_bytes())
                    .into()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        features::csrf::repository::{CsrfRepositoryError, MockCsrfRepository},
        infra::csrf::MockCsrfService,
    };

    use super::*;

    #[tokio::test]
    async fn test_verify_anonymous_valid() {
        // Arrange
        let expected_csrf_token = CsrfToken::new("csrf_token");
        let expected_csrf_cookie = CsrfCookie::new("csrf_cookie");

        let mut mock_csrf_repository = MockCsrfRepository::new();
        mock_csrf_repository
            .expect_verify_anonymous_csrf()
            .returning(|_| Box::pin(async { Ok(true) }));
        let mut mock_csrf_service = MockCsrfService::new();
        mock_csrf_service.expect_verify().return_const(true);

        let strategy =
            StatefulCsrfVerifier::new(Arc::new(mock_csrf_service), Arc::new(mock_csrf_repository));

        // Act
        let actual_is_valid = strategy
            .verify_anonymous(expected_csrf_cookie, expected_csrf_token)
            .await;

        // Assert
        assert!(actual_is_valid);
    }

    #[tokio::test]
    async fn test_verify_anonymous_invalid_state() {
        // Arrange
        let expected_csrf_token = CsrfToken::new("csrf_token");
        let expected_csrf_cookie = CsrfCookie::new("csrf_cookie");

        let mut mock_csrf_repository = MockCsrfRepository::new();
        mock_csrf_repository
            .expect_verify_anonymous_csrf()
            .returning(|_| Box::pin(async { Ok(false) }));
        let mut mock_csrf_service = MockCsrfService::new();
        mock_csrf_service.expect_verify().return_const(true);

        let strategy =
            StatefulCsrfVerifier::new(Arc::new(mock_csrf_service), Arc::new(mock_csrf_repository));

        // Act
        let actual_is_valid = strategy
            .verify_anonymous(expected_csrf_cookie, expected_csrf_token)
            .await;

        // Assert
        assert!(!actual_is_valid);
    }

    #[tokio::test]
    async fn test_verify_anonymous_storage_error() {
        // Arrange
        let expected_csrf_token = CsrfToken::new("csrf_token");
        let expected_csrf_cookie = CsrfCookie::new("csrf_cookie");

        let mut mock_csrf_repository = MockCsrfRepository::new();
        mock_csrf_repository
            .expect_verify_anonymous_csrf()
            .returning(|_| {
                Box::pin(async {
                    Err(CsrfRepositoryError::StorageError(
                        fred::error::Error::new_canceled(),
                    ))
                })
            });
        let mut mock_csrf_service = MockCsrfService::new();
        mock_csrf_service.expect_verify().return_const(true);

        let strategy =
            StatefulCsrfVerifier::new(Arc::new(mock_csrf_service), Arc::new(mock_csrf_repository));

        // Act
        let actual_is_valid = strategy
            .verify_anonymous(expected_csrf_cookie, expected_csrf_token)
            .await;

        // Assert
        assert!(!actual_is_valid);
    }

    #[tokio::test]
    async fn test_verify_anonymous_invalid() {
        // Arrange
        let expected_csrf_token = CsrfToken::new("csrf_token");
        let expected_csrf_cookie = CsrfCookie::new("csrf_cookie");

        let mut mock_csrf_repository = MockCsrfRepository::new();
        mock_csrf_repository
            .expect_verify_anonymous_csrf()
            .returning(|_| Box::pin(async { Ok(false) }));
        let mut mock_csrf_service = MockCsrfService::new();
        mock_csrf_service.expect_verify().return_const(false);

        let strategy =
            StatefulCsrfVerifier::new(Arc::new(mock_csrf_service), Arc::new(mock_csrf_repository));

        // Act
        let actual_is_valid = strategy
            .verify_anonymous(expected_csrf_cookie, expected_csrf_token)
            .await;

        // Assert
        assert!(!actual_is_valid);
    }

    #[tokio::test]
    async fn test_verify_authorized_valid() {
        // Arrange
        let expected_csrf_token = CsrfToken::new("csrf_token");
        let expected_csrf_cookie = CsrfCookie::new("csrf_cookie");
        let expected_session_csrf_token = CsrfToken::new("csrf_token");

        let mock_csrf_repository = MockCsrfRepository::new();
        let mut mock_csrf_service = MockCsrfService::new();
        mock_csrf_service.expect_verify().return_const(true);

        let strategy =
            StatefulCsrfVerifier::new(Arc::new(mock_csrf_service), Arc::new(mock_csrf_repository));

        // Act
        let actual_is_valid = strategy
            .verify_authorized(
                expected_csrf_cookie,
                expected_csrf_token,
                expected_session_csrf_token,
            )
            .await;
        // Assert
        assert!(actual_is_valid);
    }

    #[tokio::test]
    async fn test_verify_authorized_not_equal() {
        // Arrange
        let expected_csrf_token = CsrfToken::new("csrf_token");
        let expected_csrf_cookie = CsrfCookie::new("csrf_cookie");
        let expected_session_csrf_token = CsrfToken::new("not_equal");

        let mock_csrf_repository = MockCsrfRepository::new();
        let mut mock_csrf_service = MockCsrfService::new();
        mock_csrf_service.expect_verify().return_const(true);

        let strategy =
            StatefulCsrfVerifier::new(Arc::new(mock_csrf_service), Arc::new(mock_csrf_repository));

        // Act
        let actual_is_valid = strategy
            .verify_authorized(
                expected_csrf_cookie,
                expected_csrf_token,
                expected_session_csrf_token,
            )
            .await;
        // Assert
        assert!(!actual_is_valid);
    }

    #[tokio::test]
    async fn test_verify_authorized_invalid() {
        // Arrange
        let expected_csrf_token = CsrfToken::new("csrf_token");
        let expected_csrf_cookie = CsrfCookie::new("csrf_cookie");
        let expected_session_csrf_token = CsrfToken::new("csrf_token");

        let mock_csrf_repository = MockCsrfRepository::new();
        let mut mock_csrf_service = MockCsrfService::new();
        mock_csrf_service.expect_verify().return_const(false);

        let strategy =
            StatefulCsrfVerifier::new(Arc::new(mock_csrf_service), Arc::new(mock_csrf_repository));

        // Act
        let actual_is_valid = strategy
            .verify_authorized(
                expected_csrf_cookie,
                expected_csrf_token,
                expected_session_csrf_token,
            )
            .await;

        // Assert
        assert!(!actual_is_valid);
    }
}
