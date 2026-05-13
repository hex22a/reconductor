use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    constants::API_REGISTER_ENDPOINT_V1,
    features::{
        csrf::{token::TokenFeature, verify::VerifyCsrfFeature},
        session::auth::AuthFeature,
        user::{
            handler::register, login::LoginFeature, logout::LogoutFeature,
            register::RegisterFeature,
        },
    },
    state::AppState,
};

pub fn routes<R, L, O, T, A, C>(state: Arc<AppState<R, L, O, T, A, C>>) -> Router
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    O: LogoutFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
    C: VerifyCsrfFeature + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(API_REGISTER_ENDPOINT_V1, post(register::<R, L, O, T, A, C>))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::*;
    use crate::{
        features::{
            csrf::{error::CsrfError, model::CsrfTokenPair},
            session::{error::SessionError, model::UserSession},
            user::{
                dto::UserInputRequest, error::UserError, logout::LogoutFeature, model::AuthSession,
                register::RegisterFeature,
            },
        },
        state::AppState,
    };

    #[derive(Clone)]
    struct MockRegisterFeature;
    #[derive(Clone)]
    struct MockTokenFeature;
    #[derive(Clone)]
    struct MockLoginFeature;
    #[derive(Clone)]
    struct MockLogoutFeature;
    #[derive(Clone)]
    struct MockAuthFeature;
    #[derive(Clone)]
    struct MockVerifyCsrfFeature;

    impl RegisterFeature for MockRegisterFeature {
        async fn register(&self, _: String, _: String) -> Result<(), UserError> {
            Ok(())
        }
    }
    impl LoginFeature for MockLoginFeature {
        async fn login(&self, _: String, _: String, _: String) -> Result<AuthSession, UserError> {
            todo!()
        }
    }
    impl LogoutFeature for MockLogoutFeature {
        async fn logout(&self, _: String) -> Result<(), UserError> {
            todo!()
        }
    }
    impl TokenFeature for MockTokenFeature {
        async fn get_token(&self, _: Option<String>) -> Result<CsrfTokenPair, CsrfError> {
            todo!()
        }
    }
    impl AuthFeature for MockAuthFeature {
        async fn auth(&self, _: String) -> Result<UserSession, SessionError> {
            todo!()
        }
    }
    impl VerifyCsrfFeature for MockVerifyCsrfFeature {
        async fn verify_anonymous(&self, _: String, _: String) -> bool {
            todo!()
        }

        async fn verify_authorized(&self, _: String, _: String, _: String) -> bool {
            todo!()
        }
    }

    #[tokio::test]
    async fn test_register_post() {
        // Arrange
        let expected_username = "test".to_string();
        let expected_password = "password".to_string();
        let mock_register_feature = MockRegisterFeature;
        let mock_token_feature = MockTokenFeature;
        let mock_login_feature = MockLoginFeature;
        let mock_logout_feature = MockLogoutFeature;
        let mock_auth_feature = MockAuthFeature;
        let mock_verify_csrf_feature = MockVerifyCsrfFeature;
        let expected_app_state = Arc::new(AppState {
            register_feature: mock_register_feature,
            login_feature: mock_login_feature,
            logout_feature: mock_logout_feature,
            csrf_feature: mock_token_feature,
            auth_feature: mock_auth_feature,
            verify_csrf_feature: mock_verify_csrf_feature,
        });
        let expected_register_request = UserInputRequest {
            username: expected_username,
            password: expected_password,
        };
        let expected_body = Body::from(serde_json::to_vec(&expected_register_request).unwrap());
        let app = routes(expected_app_state);
        // Act
        let actual_response = app
            .oneshot(
                Request::builder()
                    .uri(API_REGISTER_ENDPOINT_V1)
                    .method("POST")
                    .header("Content-type", "application/json")
                    .body(expected_body)
                    .unwrap(),
            )
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_register_get() {
        // Arrange
        let expected_username = "test".to_string();
        let expected_password = "password".to_string();
        let mock_register_feature = MockRegisterFeature;
        let mock_token_feature = MockTokenFeature;
        let mock_login_feature = MockLoginFeature;
        let mock_logout_feature = MockLogoutFeature;
        let mock_auth_feature = MockAuthFeature;
        let mock_verify_csrf_feature = MockVerifyCsrfFeature;
        let expected_app_state = Arc::new(AppState {
            register_feature: mock_register_feature,
            login_feature: mock_login_feature,
            logout_feature: mock_logout_feature,
            csrf_feature: mock_token_feature,
            auth_feature: mock_auth_feature,
            verify_csrf_feature: mock_verify_csrf_feature,
        });
        let expected_register_request = UserInputRequest {
            username: expected_username,
            password: expected_password,
        };
        let expected_body = Body::from(serde_json::to_vec(&expected_register_request).unwrap());
        let app = routes(expected_app_state);
        // Act
        let actual_response = app
            .oneshot(
                Request::builder()
                    .uri(API_REGISTER_ENDPOINT_V1)
                    .method("GET")
                    .header("Content-type", "application/json")
                    .body(expected_body)
                    .unwrap(),
            )
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
}
