use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    constants::API_REGISTER_ENDPOINT_V1, features::user::handler::register, state::AppState,
};

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(API_REGISTER_ENDPOINT_V1, post(register))
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
            csrf::{token::MockTokenFeature, verify::MockVerifyCsrfFeature},
            host::{get::MockGetHostFeature, list::MockListHostsFeature},
            port::{get::MockGetPortFeature, list::MockListPortsFeature},
            project::{
                create::MockCreateProjectFeature, get::MockGetProjectFeature,
                list::MockListProjectsFeature,
            },
            scan::{
                create::MockCreateScanFeature, get::MockGetScanFeature, list::MockListScansFeature,
            },
            scan_run::{get::MockGetScanRunFeature, list::MockListScanRunsFeature},
            session::auth::MockAuthFeature,
            user::{
                dto::UserInputRequest, login::MockLoginFeature, logout::MockLogoutFeature,
                register::MockRegisterFeature,
            },
        },
        state::AppState,
    };

    #[tokio::test]
    async fn test_register_post() {
        // Arrange
        let expected_username = "test".to_string();
        let expected_password = "password".to_string();

        let mut mock_register_feature = MockRegisterFeature::new();
        mock_register_feature
            .expect_register()
            .returning(|_, _| Box::pin(async { Ok(()) }));
        let mock_register_feature = Arc::new(mock_register_feature);
        let mock_token_feature = Arc::new(MockTokenFeature::new());
        let mock_login_feature = Arc::new(MockLoginFeature::new());
        let mock_logout_feature = Arc::new(MockLogoutFeature::new());
        let mock_auth_feature = Arc::new(MockAuthFeature::new());
        let mock_verify_csrf_feature = Arc::new(MockVerifyCsrfFeature::new());
        let mock_create_project = Arc::new(MockCreateProjectFeature::new());
        let mock_get_project = Arc::new(MockGetProjectFeature::new());
        let mock_list_projects = Arc::new(MockListProjectsFeature::new());
        let mock_create_scan = Arc::new(MockCreateScanFeature::new());
        let mock_get_scan = Arc::new(MockGetScanFeature::new());
        let mock_list_scans = Arc::new(MockListScansFeature::new());
        let mock_get_scan_run = Arc::new(MockGetScanRunFeature::new());
        let mock_list_scan_runs = Arc::new(MockListScanRunsFeature::new());
        let mock_get_host = Arc::new(MockGetHostFeature::new());
        let mock_list_hosts = Arc::new(MockListHostsFeature::new());
        let mock_get_port = Arc::new(MockGetPortFeature::new());
        let mock_list_ports = Arc::new(MockListPortsFeature::new());

        let expected_app_state = Arc::new(AppState {
            register_feature: mock_register_feature,
            login_feature: mock_login_feature,
            logout_feature: mock_logout_feature,
            csrf_feature: mock_token_feature,
            auth_feature: mock_auth_feature,
            verify_csrf_feature: mock_verify_csrf_feature,
            create_project_feature: mock_create_project,
            get_project_feature: mock_get_project,
            list_projects_feature: mock_list_projects,
            create_scan_feature: mock_create_scan,
            get_scan_feature: mock_get_scan,
            list_scans_feature: mock_list_scans,
            get_scan_run_feature: mock_get_scan_run,
            list_scan_runs_feature: mock_list_scan_runs,
            get_host_feature: mock_get_host,
            list_hosts_feature: mock_list_hosts,
            get_port_feature: mock_get_port,
            list_ports_feature: mock_list_ports,
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

        let mut mock_register_feature = MockRegisterFeature::new();
        mock_register_feature
            .expect_register()
            .returning(|_, _| Box::pin(async { Ok(()) }));
        let mock_register_feature = Arc::new(mock_register_feature);
        let mock_token_feature = Arc::new(MockTokenFeature::new());
        let mock_login_feature = Arc::new(MockLoginFeature::new());
        let mock_logout_feature = Arc::new(MockLogoutFeature::new());
        let mock_auth_feature = Arc::new(MockAuthFeature::new());
        let mock_verify_csrf_feature = Arc::new(MockVerifyCsrfFeature::new());
        let mock_create_project = Arc::new(MockCreateProjectFeature::new());
        let mock_get_project = Arc::new(MockGetProjectFeature::new());
        let mock_list_projects = Arc::new(MockListProjectsFeature::new());
        let mock_create_scan = Arc::new(MockCreateScanFeature::new());
        let mock_get_scan = Arc::new(MockGetScanFeature::new());
        let mock_list_scans = Arc::new(MockListScansFeature::new());
        let mock_get_scan_run = Arc::new(MockGetScanRunFeature::new());
        let mock_list_scan_runs = Arc::new(MockListScanRunsFeature::new());
        let mock_get_host = Arc::new(MockGetHostFeature::new());
        let mock_list_hosts = Arc::new(MockListHostsFeature::new());
        let mock_get_port = Arc::new(MockGetPortFeature::new());
        let mock_list_ports = Arc::new(MockListPortsFeature::new());

        let expected_app_state = Arc::new(AppState {
            register_feature: mock_register_feature,
            login_feature: mock_login_feature,
            logout_feature: mock_logout_feature,
            csrf_feature: mock_token_feature,
            auth_feature: mock_auth_feature,
            verify_csrf_feature: mock_verify_csrf_feature,
            create_project_feature: mock_create_project,
            get_project_feature: mock_get_project,
            list_projects_feature: mock_list_projects,
            create_scan_feature: mock_create_scan,
            get_scan_feature: mock_get_scan,
            list_scans_feature: mock_list_scans,
            get_scan_run_feature: mock_get_scan_run,
            list_scan_runs_feature: mock_list_scan_runs,
            get_host_feature: mock_get_host,
            list_hosts_feature: mock_list_hosts,
            get_port_feature: mock_get_port,
            list_ports_feature: mock_list_ports,
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
