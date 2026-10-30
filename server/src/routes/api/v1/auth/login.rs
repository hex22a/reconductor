use std::sync::Arc;

use axum::{Router, middleware, routing::post};

use crate::{
    constants::API_LOGIN_ENDPOINT_V1,
    features::{csrf::middleware::double_submit, user::handler::login},
    state::AppState,
};

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(API_LOGIN_ENDPOINT_V1, post(login))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            double_submit,
        ))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode, header::COOKIE},
    };
    use tower::ServiceExt;

    use crate::{
        constants::{CSRF_COOKIE_NAME, CSRF_HEADER},
        domain::secret_types::{CsrfCookie, CsrfToken, SessionToken},
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
                dto::UserInputRequest, error::UserError, login::MockLoginFeature,
                logout::MockLogoutFeature, model::AuthSession, register::MockRegisterFeature,
            },
        },
    };

    use super::*;

    #[tokio::test]
    async fn test_login_post_happy_path() {
        // Arrange
        let expected_username = String::from("test");
        let expected_password = String::from("password");
        let expected_csrf_token_value = String::from("token");
        let expected_csrf_token = CsrfToken::new(expected_csrf_token_value.clone());
        let expected_csrf_cookie = CsrfCookie::new(expected_csrf_token_value.clone());
        let expected_session_token = SessionToken::new("session_id");

        let expected_auth_session = AuthSession {
            session_id: expected_session_token,
            csrf_token: expected_csrf_token,
            csrf_cookie: expected_csrf_cookie,
        };

        let mut mock_verify_csrf_feature = MockVerifyCsrfFeature::new();
        mock_verify_csrf_feature
            .expect_verify_anonymous()
            .returning(|_, _| Box::pin(async { true }));

        let mut mock_login_feature = MockLoginFeature::new();
        mock_login_feature.expect_login().returning(move |_, _, _| {
            let auth_session = expected_auth_session.clone();
            Box::pin(async { Ok(auth_session) })
        });

        let mock_register_feature = Arc::new(MockRegisterFeature::new());
        let mock_token_feature = Arc::new(MockTokenFeature::new());
        let mock_login_feature = Arc::new(mock_login_feature);
        let mock_logout_feature = Arc::new(MockLogoutFeature::new());
        let mock_auth_feature = Arc::new(MockAuthFeature::new());
        let mock_verify_csrf_feature = Arc::new(mock_verify_csrf_feature);
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

        let expected_login_request = UserInputRequest {
            username: expected_username,
            password: expected_password,
        };

        let expected_body = Body::from(serde_json::to_vec(&expected_login_request).unwrap());
        let app = routes(expected_app_state);

        // Act
        let actual_response = app
            .oneshot(
                Request::builder()
                    .uri(API_LOGIN_ENDPOINT_V1)
                    .method("POST")
                    .header("Content-type", "application/json")
                    .header(CSRF_HEADER, &expected_csrf_token_value)
                    .header(
                        COOKIE,
                        format!("{}={}", CSRF_COOKIE_NAME, expected_csrf_token_value),
                    )
                    .body(expected_body)
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_login_post_internal_error() {
        // Arrange
        let expected_username = String::from("test");
        let expected_password = String::from("password");
        let expected_csrf_token_value = String::from("token");

        let mut mock_verify_csrf_feature = MockVerifyCsrfFeature::new();
        mock_verify_csrf_feature
            .expect_verify_anonymous()
            .returning(|_, _| Box::pin(async { true }));

        let mut mock_login_feature = MockLoginFeature::new();
        mock_login_feature
            .expect_login()
            .returning(|_, _, _| Box::pin(async { Err(UserError::Interntal) }));

        let mock_register_feature = Arc::new(MockRegisterFeature::new());
        let mock_token_feature = Arc::new(MockTokenFeature::new());
        let mock_login_feature = Arc::new(mock_login_feature);
        let mock_logout_feature = Arc::new(MockLogoutFeature::new());
        let mock_auth_feature = Arc::new(MockAuthFeature::new());
        let mock_verify_csrf_feature = Arc::new(mock_verify_csrf_feature);
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

        let expected_login_request = UserInputRequest {
            username: expected_username,
            password: expected_password,
        };

        let expected_body = Body::from(serde_json::to_vec(&expected_login_request).unwrap());
        let app = routes(expected_app_state);

        // Act
        let actual_response = app
            .oneshot(
                Request::builder()
                    .uri(API_LOGIN_ENDPOINT_V1)
                    .method("POST")
                    .header("Content-type", "application/json")
                    .header(CSRF_HEADER, &expected_csrf_token_value)
                    .header(
                        COOKIE,
                        format!("{}={}", CSRF_COOKIE_NAME, expected_csrf_token_value),
                    )
                    .body(expected_body)
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_login_post_wrong_csrf() {
        // Arrange
        let expected_username = String::from("test");
        let expected_password = String::from("password");
        let expected_csrf_token_value = String::from("token");
        let expected_csrf_token = CsrfToken::new(expected_csrf_token_value.clone());
        let expected_csrf_cookie = CsrfCookie::new(expected_csrf_token_value.clone());
        let expected_session_token = SessionToken::new("session_id");

        let expected_auth_session = AuthSession {
            session_id: expected_session_token,
            csrf_token: expected_csrf_token,
            csrf_cookie: expected_csrf_cookie,
        };

        let mut mock_verify_csrf_feature = MockVerifyCsrfFeature::new();
        mock_verify_csrf_feature
            .expect_verify_anonymous()
            .returning(|_, _| Box::pin(async { false }));

        let mut mock_login_feature = MockLoginFeature::new();
        mock_login_feature.expect_login().returning(move |_, _, _| {
            let auth_session = expected_auth_session.clone();
            Box::pin(async { Ok(auth_session) })
        });

        let mock_register_feature = Arc::new(MockRegisterFeature::new());
        let mock_token_feature = Arc::new(MockTokenFeature::new());
        let mock_login_feature = Arc::new(mock_login_feature);
        let mock_logout_feature = Arc::new(MockLogoutFeature::new());
        let mock_auth_feature = Arc::new(MockAuthFeature::new());
        let mock_verify_csrf_feature = Arc::new(mock_verify_csrf_feature);
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

        let expected_login_request = UserInputRequest {
            username: expected_username,
            password: expected_password,
        };

        let expected_body = Body::from(serde_json::to_vec(&expected_login_request).unwrap());
        let app = routes(expected_app_state);

        // Act
        let actual_response = app
            .oneshot(
                Request::builder()
                    .uri(API_LOGIN_ENDPOINT_V1)
                    .method("POST")
                    .header("Content-type", "application/json")
                    .header(CSRF_HEADER, &expected_csrf_token_value)
                    .header(
                        COOKIE,
                        format!("{}={}", CSRF_COOKIE_NAME, expected_csrf_token_value),
                    )
                    .body(expected_body)
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_login_post_missing_header() {
        // Arrange
        let expected_username = String::from("test");
        let expected_password = String::from("password");
        let expected_csrf_token_value = String::from("token");
        let expected_csrf_token = CsrfToken::new(expected_csrf_token_value.clone());
        let expected_csrf_cookie = CsrfCookie::new(expected_csrf_token_value.clone());
        let expected_session_token = SessionToken::new("session_id");

        let expected_auth_session = AuthSession {
            session_id: expected_session_token,
            csrf_token: expected_csrf_token,
            csrf_cookie: expected_csrf_cookie,
        };

        let mut mock_verify_csrf_feature = MockVerifyCsrfFeature::new();
        mock_verify_csrf_feature
            .expect_verify_anonymous()
            .returning(|_, _| Box::pin(async { true }));

        let mut mock_login_feature = MockLoginFeature::new();
        mock_login_feature.expect_login().returning(move |_, _, _| {
            let auth_session = expected_auth_session.clone();
            Box::pin(async { Ok(auth_session) })
        });

        let mock_register_feature = Arc::new(MockRegisterFeature::new());
        let mock_token_feature = Arc::new(MockTokenFeature::new());
        let mock_login_feature = Arc::new(mock_login_feature);
        let mock_logout_feature = Arc::new(MockLogoutFeature::new());
        let mock_auth_feature = Arc::new(MockAuthFeature::new());
        let mock_verify_csrf_feature = Arc::new(mock_verify_csrf_feature);
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

        let expected_login_request = UserInputRequest {
            username: expected_username,
            password: expected_password,
        };

        let expected_body = Body::from(serde_json::to_vec(&expected_login_request).unwrap());
        let app = routes(expected_app_state);

        // Act
        let actual_response = app
            .oneshot(
                Request::builder()
                    .uri(API_LOGIN_ENDPOINT_V1)
                    .method("POST")
                    .header("Content-type", "application/json")
                    .header(
                        COOKIE,
                        format!("{}={}", CSRF_COOKIE_NAME, expected_csrf_token_value),
                    )
                    .body(expected_body)
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_login_post_missing_cookie() {
        // Arrange
        let expected_username = String::from("test");
        let expected_password = String::from("password");
        let expected_csrf_token_value = String::from("token");
        let expected_csrf_token = CsrfToken::new(expected_csrf_token_value.clone());
        let expected_csrf_cookie = CsrfCookie::new(expected_csrf_token_value.clone());
        let expected_session_token = SessionToken::new("session_id");

        let expected_auth_session = AuthSession {
            session_id: expected_session_token,
            csrf_token: expected_csrf_token,
            csrf_cookie: expected_csrf_cookie,
        };

        let mut mock_verify_csrf_feature = MockVerifyCsrfFeature::new();
        mock_verify_csrf_feature
            .expect_verify_anonymous()
            .returning(|_, _| Box::pin(async { true }));

        let mut mock_login_feature = MockLoginFeature::new();
        mock_login_feature.expect_login().returning(move |_, _, _| {
            let auth_session = expected_auth_session.clone();
            Box::pin(async { Ok(auth_session) })
        });

        let mock_register_feature = Arc::new(MockRegisterFeature::new());
        let mock_token_feature = Arc::new(MockTokenFeature::new());
        let mock_login_feature = Arc::new(mock_login_feature);
        let mock_logout_feature = Arc::new(MockLogoutFeature::new());
        let mock_auth_feature = Arc::new(MockAuthFeature::new());
        let mock_verify_csrf_feature = Arc::new(mock_verify_csrf_feature);
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

        let expected_login_request = UserInputRequest {
            username: expected_username,
            password: expected_password,
        };

        let expected_body = Body::from(serde_json::to_vec(&expected_login_request).unwrap());
        let app = routes(expected_app_state);

        // Act
        let actual_response = app
            .oneshot(
                Request::builder()
                    .uri(API_LOGIN_ENDPOINT_V1)
                    .method("POST")
                    .header("Content-type", "application/json")
                    .header(CSRF_HEADER, &expected_csrf_token_value)
                    .body(expected_body)
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_response.status(), StatusCode::FORBIDDEN);
    }
}
