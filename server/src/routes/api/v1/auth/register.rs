use axum::{Router, routing::post};

use crate::{
    constants::API_REGISTER_ENDPOINT_V1,
    features::{
        csrf::token::TokenFeature,
        user::{handler::handle, register::RegisterFeature},
    },
    state::AppState,
};

pub fn routes<R, T>(state: AppState<R, T>) -> Router
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(API_REGISTER_ENDPOINT_V1, post(handle::<R, T>))
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
        domain::error::ServerError,
        features::{
            csrf::model::CsrfTokenPair,
            user::{dto::RegisterUserRequest, register::RegisterFeature},
        },
        state::AppState,
    };

    #[derive(Clone)]
    struct MockRegisterFeature;
    #[derive(Clone)]
    struct MockTokenFeature;
    impl RegisterFeature for MockRegisterFeature {
        async fn register(
            &self,
            _: String,
            _: String,
        ) -> Result<(), crate::domain::error::ServerError> {
            Ok(())
        }
    }
    impl TokenFeature for MockTokenFeature {
        async fn get_token(&mut self, _: Option<String>) -> Result<CsrfTokenPair, ServerError> {
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
        let expected_app_state = AppState {
            register_feature: mock_register_feature,
            csrf_feature: mock_token_feature,
        };
        let expected_register_request = RegisterUserRequest {
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
        let expected_app_state = AppState {
            register_feature: mock_register_feature,
            csrf_feature: mock_token_feature,
        };
        let expected_register_request = RegisterUserRequest {
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
