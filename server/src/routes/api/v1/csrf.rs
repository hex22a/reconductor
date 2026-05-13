use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    constants::API_CSRF_ENDPOINT_V1,
    features::{
        csrf::{handler::handle, token::TokenFeature, verify::VerifyCsrfFeature},
        session::auth::AuthFeature,
        user::{login::LoginFeature, logout::LogoutFeature, register::RegisterFeature},
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
        .route(API_CSRF_ENDPOINT_V1, get(handle::<R, L, O, T, A, C>))
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
        constants::USER_SESSION_COOKIE_NAME,
        features::{
            csrf::{error::CsrfError, model::CsrfTokenPair, token::TokenFeature},
            session::{error::SessionError, model::UserSession},
            user::{error::UserError, model::AuthSession, register::RegisterFeature},
        },
    };

    #[derive(Clone)]
    struct MockRegisterFeature;
    #[derive(Clone)]
    struct MockTokenFeature {
        return_value: CsrfTokenPair,
    }
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
            todo!()
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
            Ok(self.return_value.clone())
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
    async fn test_csrf_get() {
        // Arrange
        let expected_session_cookie = "session_cookie".to_string();
        let expected_cookie_header =
            format!("{}={}", USER_SESSION_COOKIE_NAME, expected_session_cookie);
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie_value = "csrf_cookie".to_string();
        let expected_csrf_token_pair = CsrfTokenPair {
            token: expected_csrf_token,
            cookie_value: expected_csrf_cookie_value,
        };
        let mock_register_feature = MockRegisterFeature;
        let mock_login_feature = MockLoginFeature;
        let mock_logout_feature = MockLogoutFeature;
        let mock_auth_feature = MockAuthFeature;
        let mock_token_feature = MockTokenFeature {
            return_value: expected_csrf_token_pair,
        };
        let mock_verify_csrf_feature = MockVerifyCsrfFeature;
        let expected_app_state = Arc::new(AppState {
            register_feature: mock_register_feature,
            login_feature: mock_login_feature,
            logout_feature: mock_logout_feature,
            csrf_feature: mock_token_feature,
            auth_feature: mock_auth_feature,
            verify_csrf_feature: mock_verify_csrf_feature,
        });
        let app = routes(expected_app_state);
        // Act
        let actual_response = app
            .oneshot(
                Request::builder()
                    .uri(API_CSRF_ENDPOINT_V1)
                    .method("GET")
                    .header("Cookie", expected_cookie_header)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_csrf_post() {
        // Arrange
        let expected_session_cookie = "session_cookie".to_string();
        let expected_cookie_header =
            format!("{}={}", USER_SESSION_COOKIE_NAME, expected_session_cookie);
        let expected_csrf_token = "csrf_token".to_string();
        let expected_csrf_cookie_value = "csrf_cookie".to_string();
        let expected_csrf_token_pair = CsrfTokenPair {
            token: expected_csrf_token,
            cookie_value: expected_csrf_cookie_value,
        };
        let mock_register_feature = MockRegisterFeature;
        let mock_login_feature = MockLoginFeature;
        let mock_logout_feature = MockLogoutFeature;
        let mock_auth_feature = MockAuthFeature;
        let mock_token_feature = MockTokenFeature {
            return_value: expected_csrf_token_pair,
        };
        let mock_verify_csrf_feature = MockVerifyCsrfFeature;
        let expected_app_state = Arc::new(AppState {
            register_feature: mock_register_feature,
            login_feature: mock_login_feature,
            logout_feature: mock_logout_feature,
            csrf_feature: mock_token_feature,
            auth_feature: mock_auth_feature,
            verify_csrf_feature: mock_verify_csrf_feature,
        });
        let app = routes(expected_app_state);
        // Act
        let actual_response = app
            .oneshot(
                Request::builder()
                    .uri(API_CSRF_ENDPOINT_V1)
                    .method("POST")
                    .header("Cookie", expected_cookie_header)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
}
