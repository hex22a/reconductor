use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    constants::API_CSRF_ENDPOINT_V1,
    features::{
        csrf::{handler::handle, token::TokenFeature},
        user::register::RegisterFeature,
    },
    state::AppState,
};

pub fn routes<R, T>(state: Arc<AppState<R, T>>) -> Router
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(API_CSRF_ENDPOINT_V1, get(handle::<R, T>))
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
        domain::error::ServerError,
        features::{
            csrf::{model::CsrfTokenPair, token::TokenFeature},
            user::register::RegisterFeature,
        },
    };

    #[derive(Clone)]
    struct MockRegisterFeature;
    #[derive(Clone)]
    struct MockTokenFeature {
        return_value: CsrfTokenPair,
    }
    impl RegisterFeature for MockRegisterFeature {
        async fn register(&self, _: String, _: String) -> Result<(), ServerError> {
            todo!()
        }
    }
    impl TokenFeature for MockTokenFeature {
        async fn get_token(&self, _: Option<String>) -> Result<CsrfTokenPair, ServerError> {
            Ok(self.return_value.clone())
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
            cookie_value: Some(expected_csrf_cookie_value),
        };
        let mock_register_feature = MockRegisterFeature;
        let mock_token_feature = MockTokenFeature {
            return_value: expected_csrf_token_pair,
        };
        let expected_app_state = Arc::new(AppState {
            register_feature: mock_register_feature,
            csrf_feature: mock_token_feature,
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
            cookie_value: Some(expected_csrf_cookie_value),
        };
        let mock_register_feature = MockRegisterFeature;
        let mock_token_feature = MockTokenFeature {
            return_value: expected_csrf_token_pair,
        };
        let expected_app_state = Arc::new(AppState {
            register_feature: mock_register_feature,
            csrf_feature: mock_token_feature,
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
