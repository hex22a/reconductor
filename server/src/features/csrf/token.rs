use crate::{
    domain::error::ServerError,
    features::csrf::model::CsrfTokenPair,
    infra::csrf::CsrfService,
    persistence::kv::{csrf::CsrfRepository, session::SessionRepository},
};

pub trait TokenFeature {
    fn get_token(
        &self,
        session_token: Option<String>,
    ) -> impl Future<Output = Result<CsrfTokenPair, ServerError>> + Send;
}

struct CsrfTokenFeature<R: SessionRepository, C: CsrfRepository, S: CsrfService> {
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
    async fn get_token(&self, session_token: Option<String>) -> Result<CsrfTokenPair, ServerError> {
        let user_session = self
            .session_repository
            .get_user_session(session_token.expect("No seesion token"))
            .await?;
        Ok(CsrfTokenPair {
            token: user_session.csrf_token,
            cookie_value: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::{
        infra::csrf::{CsrfService, CsrfServiceError},
        persistence::kv::{
            csrf::CsrfError,
            session::{SessionError, SessionRepository, UserSession},
        },
    };

    struct MockSessionRepository {
        return_value: UserSession,
    }
    struct MockCsrfRepository;
    struct MockCsrfService {
        return_value: (String, String),
    }
    impl SessionRepository for MockSessionRepository {
        async fn create_user_session(&self, _: UserSession) -> Result<(), SessionError> {
            todo!()
        }

        async fn get_user_session(&self, _: String) -> Result<UserSession, SessionError> {
            Ok(self.return_value.clone())
        }

        async fn delete_user_session(&self, _: String) -> Result<(), SessionError> {
            todo!()
        }
    }
    impl CsrfRepository for MockCsrfRepository {
        async fn create_anonymous_csrf(&self, _: &str) -> Result<(), CsrfError> {
            todo!()
        }

        async fn verify_anonymous_csrf(&self, _: &str) -> Result<bool, CsrfError> {
            todo!()
        }

        async fn delete_anonymous_csrf(&self, _: &str) -> Result<(), CsrfError> {
            todo!()
        }
    }
    impl CsrfService for MockCsrfService {
        fn generate(&mut self, _: u64) -> Result<(String, String), CsrfServiceError> {
            Ok(self.return_value.clone())
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
            return_value: expected_user_session,
        };
        let mock_csrf_repository = MockCsrfRepository;
        let mock_csrf_service = MockCsrfService {
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
}
