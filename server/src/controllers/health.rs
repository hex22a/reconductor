use axum::{Json, http::StatusCode};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct HealthResponse {
    healthy: bool,
}
pub async fn get_health() -> (StatusCode, Json<HealthResponse>) {
    (StatusCode::OK, Json(HealthResponse { healthy: true }))
}

#[cfg(test)]
mod tests {
    use axum::{Json, http::StatusCode};

    use crate::controllers::health::{HealthResponse, get_health};

    #[tokio::test]
    async fn test_get_health() {
        // Arrange
        let expected_health_response = HealthResponse { healthy: true };
        let expected_health = (StatusCode::OK, Json(expected_health_response));
        // Act
        let actual_health = get_health().await;
        // Assert
        assert_eq!(actual_health.0, expected_health.0);
        assert_eq!(actual_health.1.0, expected_health.1.0);
    }
}
