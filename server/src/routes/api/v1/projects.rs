use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    constants::API_PROJECTS_ENDPOINT_V1,
    features::{
        csrf::middleware::session_based,
        project::handler::{create, get_project, list},
        session::middleware::session_middleware,
    },
    state::AppState,
};

mod scans;

pub fn routes(state: Arc<AppState>) -> Router {
    let scan_routes = scans::routes();
    let project_routes = Router::new()
        .route("/{project_id}", get(get_project))
        .nest("/{project_id}", scan_routes);
    let inner_routes = Router::new()
        .route("/", post(create))
        .route("/", get(list))
        .merge(project_routes)
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            session_based,
        ))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            session_middleware,
        ));

    Router::new()
        .nest(API_PROJECTS_ENDPOINT_V1, inner_routes)
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode, header::COOKIE},
    };
    use time::macros::datetime;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{
        constants::{CSRF_COOKIE_NAME, CSRF_HEADER, USER_SESSION_COOKIE_NAME},
        domain::{
            secret_types::{CsrfCookie, CsrfToken, SessionToken},
            types::{ProjectId, ProjectName, UserId, Username},
        },
        features::{
            csrf::{token::MockTokenFeature, verify::MockVerifyCsrfFeature},
            host::{get::MockGetHostFeature, list::MockListHostsFeature},
            port::{get::MockGetPortFeature, list::MockListPortsFeature},
            project::{
                create::MockCreateProjectFeature,
                dto::{CreateProjctRequest, ProjectDto},
                get::MockGetProjectFeature,
                list::MockListProjectsFeature,
            },
            scan::{
                create::MockCreateScanFeature, get::MockGetScanFeature, list::MockListScansFeature,
            },
            scan_run::{get::MockGetScanRunFeature, list::MockListScanRunsFeature},
            session::{auth::MockAuthFeature, model::UserSession},
            user::{
                login::MockLoginFeature, logout::MockLogoutFeature, register::MockRegisterFeature,
            },
        },
    };

    use super::*;

    #[tokio::test]
    async fn test_create_project_happy_path() {
        // Arrange
        let expected_project_id = ProjectId::new(Uuid::now_v7());
        let expected_project_name_string = String::from("test");
        let expected_project_name = ProjectName::new(expected_project_name_string.clone());
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_session_cookie_value = String::from("session_cookie");
        let expected_session_cookie = SessionToken::new(expected_session_cookie_value.clone());
        let expected_csrf_token_value = String::from("token");
        let expected_user_id = UserId::new(Uuid::now_v7());
        let expected_username = Username::new("user");
        let expected_csrf_token = CsrfToken::new("csrf_token");
        let expected_csrf_cookie = CsrfCookie::new("csrf_cookie");
        let expected_user_session = UserSession {
            token: expected_session_cookie,
            user_id: expected_user_id,
            username: expected_username,
            csrf_token: expected_csrf_token,
            csrf_cookie: expected_csrf_cookie,
        };
        let expected_project_dto = ProjectDto {
            id: expected_project_id,
            name: expected_project_name,
            created_at: expected_created_at,
        };

        let mut mock_auth_feature = MockAuthFeature::new();
        mock_auth_feature.expect_auth().returning(move |_| {
            let user_session = expected_user_session.clone();
            Box::pin(async { Ok(user_session) })
        });

        let mut mock_verify_csrf_feature = MockVerifyCsrfFeature::new();
        mock_verify_csrf_feature
            .expect_verify_authorized()
            .returning(|_, _, _| Box::pin(async { true }));

        let mut mock_create_project_feature = MockCreateProjectFeature::new();
        mock_create_project_feature
            .expect_create()
            .returning(move |_, _| {
                let project_dto = expected_project_dto.clone();
                Box::pin(async { Ok(project_dto) })
            });

        let mock_register_feature = Arc::new(MockRegisterFeature::new());
        let mock_token_feature = Arc::new(MockTokenFeature::new());
        let mock_login_feature = Arc::new(MockLoginFeature::new());
        let mock_logout_feature = Arc::new(MockLogoutFeature::new());
        let mock_auth_feature = Arc::new(mock_auth_feature);
        let mock_verify_csrf_feature = Arc::new(mock_verify_csrf_feature);
        let mock_create_project = Arc::new(mock_create_project_feature);
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
        let expected_create_project_request = CreateProjctRequest {
            name: expected_project_name_string,
        };

        let expected_body =
            Body::from(serde_json::to_vec(&expected_create_project_request).unwrap());
        let app = routes(expected_app_state);

        // Act
        let actual_response = app
            .oneshot(
                Request::builder()
                    .uri(API_PROJECTS_ENDPOINT_V1)
                    .method("POST")
                    .header("Content-type", "application/json")
                    .header(CSRF_HEADER, &expected_csrf_token_value)
                    .header(
                        COOKIE,
                        format!("{}={}", CSRF_COOKIE_NAME, expected_csrf_token_value),
                    )
                    .header(
                        COOKIE,
                        format!(
                            "{}={}",
                            USER_SESSION_COOKIE_NAME, expected_session_cookie_value
                        ),
                    )
                    .body(expected_body)
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_create_project_no_csrf() {
        // Arrange
        let expected_project_id = ProjectId::new(Uuid::now_v7());
        let expected_project_name_string = String::from("test");
        let expected_project_name = ProjectName::new(expected_project_name_string.clone());
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_session_cookie_value = String::from("session_cookie");
        let expected_session_cookie = SessionToken::new(expected_session_cookie_value.clone());
        let expected_csrf_token_value = String::from("token");
        let expected_user_id = UserId::new(Uuid::now_v7());
        let expected_username = Username::new("user");
        let expected_csrf_token = CsrfToken::new("csrf_token");
        let expected_csrf_cookie = CsrfCookie::new("csrf_cookie");
        let expected_user_session = UserSession {
            token: expected_session_cookie,
            user_id: expected_user_id,
            username: expected_username,
            csrf_token: expected_csrf_token,
            csrf_cookie: expected_csrf_cookie,
        };
        let expected_project_dto = ProjectDto {
            id: expected_project_id,
            name: expected_project_name,
            created_at: expected_created_at,
        };

        let mut mock_auth_feature = MockAuthFeature::new();
        mock_auth_feature.expect_auth().returning(move |_| {
            let user_session = expected_user_session.clone();
            Box::pin(async { Ok(user_session) })
        });

        let mut mock_verify_csrf_feature = MockVerifyCsrfFeature::new();
        mock_verify_csrf_feature
            .expect_verify_authorized()
            .returning(|_, _, _| Box::pin(async { true }));

        let mut mock_create_project_feature = MockCreateProjectFeature::new();
        mock_create_project_feature
            .expect_create()
            .returning(move |_, _| {
                let project_dto = expected_project_dto.clone();
                Box::pin(async { Ok(project_dto) })
            });

        let mock_register_feature = Arc::new(MockRegisterFeature::new());
        let mock_token_feature = Arc::new(MockTokenFeature::new());
        let mock_login_feature = Arc::new(MockLoginFeature::new());
        let mock_logout_feature = Arc::new(MockLogoutFeature::new());
        let mock_auth_feature = Arc::new(mock_auth_feature);
        let mock_verify_csrf_feature = Arc::new(mock_verify_csrf_feature);
        let mock_create_project = Arc::new(mock_create_project_feature);
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
        let expected_create_project_request = CreateProjctRequest {
            name: expected_project_name_string,
        };

        let expected_body =
            Body::from(serde_json::to_vec(&expected_create_project_request).unwrap());
        let app = routes(expected_app_state);

        // Act
        let actual_response = app
            .oneshot(
                Request::builder()
                    .uri(API_PROJECTS_ENDPOINT_V1)
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
        assert_eq!(actual_response.status(), StatusCode::UNAUTHORIZED);
    }
}
