use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{constants::API_CSRF_ENDPOINT_V1, features::csrf::handler::handle, state::AppState};

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(API_CSRF_ENDPOINT_V1, get(handle))
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
            csrf::{
                error::CsrfError,
                model::CsrfTokenPair,
                token::{MockTokenFeature, TokenFeature},
                verify::MockVerifyCsrfFeature,
            },
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
                login::MockLoginFeature, logout::MockLogoutFeature, register::MockRegisterFeature,
            },
        },
    };

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

        let mut mock_token_feature = MockTokenFeature::new();
        mock_token_feature.expect_get_token().returning(move |_| {
            let csrf_token_pair = expected_csrf_token_pair.clone();
            Box::pin(async { Ok(csrf_token_pair) })
        });
        let mock_register_feature = Arc::new(MockRegisterFeature::new());
        let mock_login_feature = Arc::new(MockLoginFeature::new());
        let mock_logout_feature = Arc::new(MockLogoutFeature::new());
        let mock_auth_feature = Arc::new(MockAuthFeature::new());
        let mock_token_feature = Arc::new(mock_token_feature);
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
        let mock_list_ports = Arc::new(MockListPortsFeature::new());
        let mock_get_port = Arc::new(MockGetPortFeature::new());

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

        let mut mock_token_feature = MockTokenFeature::new();
        mock_token_feature.expect_get_token().returning(move |_| {
            let csrf_token_pair = expected_csrf_token_pair.clone();
            Box::pin(async { Ok(csrf_token_pair) })
        });
        let mock_register_feature = Arc::new(MockRegisterFeature::new());
        let mock_login_feature = Arc::new(MockLoginFeature::new());
        let mock_logout_feature = Arc::new(MockLogoutFeature::new());
        let mock_auth_feature = Arc::new(MockAuthFeature::new());
        let mock_token_feature = Arc::new(mock_token_feature);
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
        let mock_list_ports = Arc::new(MockListPortsFeature::new());
        let mock_get_port = Arc::new(MockGetPortFeature::new());

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
