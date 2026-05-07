use axum::{Router, routing::post};

use crate::{
    constants::API_REGISTER_ENDPOINT_V1,
    features::user::{handler::register, register::RegisterFeature},
    state::AppState,
};

pub fn routes<R>(state: AppState<R>) -> Router
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(API_REGISTER_ENDPOINT_V1, post(register::<R>))
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
        features::user::{dto::RegisterUserRequest, register::RegisterFeature},
        state::AppState,
    };

    #[derive(Clone)]
    struct MockRegisterFeature;
    impl RegisterFeature for MockRegisterFeature {
        async fn register(
            &self,
            _: String,
            _: String,
        ) -> Result<(), crate::domain::error::ServerError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_register_post() {
        // Arrange
        let expected_username = "test".to_string();
        let expected_password = "password".to_string();
        let mock_register_feature = MockRegisterFeature;
        let expected_app_state = AppState {
            register_feature: mock_register_feature,
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
        let expected_app_state = AppState {
            register_feature: mock_register_feature,
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
