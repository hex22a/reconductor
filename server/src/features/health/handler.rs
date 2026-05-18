use axum::{Json, http::StatusCode};

use crate::features::health::dto::HealthResponse;

pub(crate) async fn handle() -> (StatusCode, Json<HealthResponse>) {
    (StatusCode::OK, Json(HealthResponse { healthy: true }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Json, http::StatusCode};

    #[tokio::test]
    async fn test_get_health() {
        // Arrange
        let expected_health_response = HealthResponse { healthy: true };
        let expected_health = (StatusCode::OK, Json(expected_health_response));
        // Act
        let actual_health = handle().await;
        // Assert
        assert_eq!(actual_health.0, expected_health.0);
        assert_eq!(actual_health.1.0, expected_health.1.0);
    }
}
