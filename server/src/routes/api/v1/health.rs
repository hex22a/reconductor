use crate::constants::API_HEALTH_ENDPOINT_V1;
use crate::features::health::handler::handle;
use axum::Router;
use axum::routing::get;

pub(crate) fn routes() -> Router {
    Router::new().route(API_HEALTH_ENDPOINT_V1, get(handle))
}

#[cfg(test)]
mod tests {
    use crate::{constants::API_HEALTH_ENDPOINT_V1, routes::api::v1::health::routes};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_routes_get() {
        // Arrange
        let app = routes();
        // Act
        let response = app
            .oneshot(
                Request::builder()
                    .uri(API_HEALTH_ENDPOINT_V1)
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // Assert
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_health_routes_post() {
        // Arrange
        let app = routes();
        // Act
        let response = app
            .oneshot(
                Request::builder()
                    .uri(API_HEALTH_ENDPOINT_V1)
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // Assert
        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
}
