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
                error::CsrfError, model::CsrfTokenPair, token::TokenFeature,
                verify::VerifyCsrfFeature,
            },
            project::{
                create::CreateProjectFeature, dto::ProjectDto, error::ProjectError,
                get::GetProjectFeature, list::ListProjectsFeature,
            },
            scan::{
                create::CreateScanFeature, dto::ScanDto, error::ScanError, get::GetScanFeature,
                list::ListScansFeature,
            },
            scan_run::{
                dto::ScanRunDto, error::ScanRunError, get::GetScanRunFeature,
                list::ListScanRunsFeature,
            },
            session::{auth::AuthFeature, error::SessionError, model::UserSession},
            user::{
                error::UserError, login::LoginFeature, logout::LogoutFeature, model::AuthSession,
                register::RegisterFeature,
            },
        },
        transport::pagination::Page,
    };

    #[derive(Clone)]
    struct MockRegisterFeature;
    #[derive(Clone)]
    struct MockTokenFeature {
        return_value: CsrfTokenPair,
    }
    #[derive(Clone)]
    struct MockLoginFeature;
    #[derive(Clone)]
    struct MockLogoutFeature;
    #[derive(Clone)]
    struct MockAuthFeature;
    #[derive(Clone)]
    struct MockVerifyCsrfFeature;
    #[derive(Clone)]
    struct MockCreateProject;
    #[derive(Clone)]
    struct MockGetProject;
    #[derive(Clone)]
    struct MockListProjects;
    #[derive(Clone)]
    struct MockCreateScan;
    #[derive(Clone)]
    struct MockGetScan;
    #[derive(Clone)]
    struct MockListScans;
    #[derive(Clone)]
    struct MockGetScanRun;
    #[derive(Clone)]
    struct MockListScanRuns;

    impl RegisterFeature for MockRegisterFeature {
        fn register(
            &self,
            _: String,
            _: String,
        ) -> std::pin::Pin<Box<dyn Future<Output = Result<(), UserError>> + Send + '_>> {
            todo!()
        }
    }
    impl LoginFeature for MockLoginFeature {
        fn login(
            &self,
            _: String,
            _: String,
            _: String,
        ) -> std::pin::Pin<Box<dyn Future<Output = Result<AuthSession, UserError>> + Send + '_>>
        {
            todo!()
        }
    }
    impl LogoutFeature for MockLogoutFeature {
        fn logout<'a>(
            &'a self,
            _: &'a str,
        ) -> std::pin::Pin<Box<dyn Future<Output = Result<(), UserError>> + Send + 'a>> {
            todo!()
        }
    }
    impl TokenFeature for MockTokenFeature {
        fn get_token<'a>(
            &'a self,
            _: Option<&'a str>,
        ) -> std::pin::Pin<Box<dyn Future<Output = Result<CsrfTokenPair, CsrfError>> + Send + 'a>>
        {
            Box::pin(async { Ok(self.return_value.clone()) })
        }
    }
    impl AuthFeature for MockAuthFeature {
        fn auth<'a>(
            &'a self,
            _: &'a str,
        ) -> std::pin::Pin<Box<dyn Future<Output = Result<UserSession, SessionError>> + Send + 'a>>
        {
            todo!()
        }
    }
    impl VerifyCsrfFeature for MockVerifyCsrfFeature {
        fn verify_anonymous(
            &self,
            _: String,
            _: String,
        ) -> std::pin::Pin<Box<dyn Future<Output = bool> + Send + '_>> {
            todo!()
        }

        fn verify_authorized(
            &self,
            _: String,
            _: String,
            _: String,
        ) -> std::pin::Pin<Box<dyn Future<Output = bool> + Send + '_>> {
            todo!()
        }
    }
    impl CreateProjectFeature for MockCreateProject {
        fn create(
            &self,
            _: uuid::Uuid,
            _: String,
        ) -> std::pin::Pin<Box<dyn Future<Output = Result<ProjectDto, ProjectError>> + Send + '_>>
        {
            todo!()
        }
    }
    impl GetProjectFeature for MockGetProject {
        fn get(
            &self,
            _: uuid::Uuid,
            _: uuid::Uuid,
        ) -> std::pin::Pin<Box<dyn Future<Output = Result<ProjectDto, ProjectError>> + Send + '_>>
        {
            todo!()
        }
    }
    impl ListProjectsFeature for MockListProjects {
        fn list<'a>(
            &'a self,
            _: &'a uuid::Uuid,
            _: Option<&'a str>,
        ) -> std::pin::Pin<
            Box<dyn Future<Output = Result<Page<ProjectDto>, ProjectError>> + Send + 'a>,
        > {
            todo!()
        }
    }

    impl CreateScanFeature for MockCreateScan {
        fn create(
            &self,
            _: uuid::Uuid,
            _: sqlx::types::ipnetwork::IpNetwork,
            _: Option<cron::Schedule>,
        ) -> std::pin::Pin<Box<dyn Future<Output = Result<ScanDto, ScanError>> + Send + '_>>
        {
            todo!()
        }
    }
    impl GetScanFeature for MockGetScan {
        fn get(
            &self,
            _: uuid::Uuid,
        ) -> std::pin::Pin<Box<dyn Future<Output = Result<ScanDto, ScanError>> + Send + '_>>
        {
            todo!()
        }
    }
    impl ListScansFeature for MockListScans {
        fn list<'a>(
            &'a self,
            _: &'a uuid::Uuid,
            _: Option<&'a str>,
        ) -> std::pin::Pin<Box<dyn Future<Output = Result<Page<ScanDto>, ScanError>> + Send + 'a>>
        {
            todo!()
        }
    }
    impl GetScanRunFeature for MockGetScanRun {
        fn get(
            &self,
            _: uuid::Uuid,
        ) -> std::pin::Pin<Box<dyn Future<Output = Result<ScanRunDto, ScanRunError>> + Send + '_>>
        {
            todo!()
        }
    }
    impl ListScanRunsFeature for MockListScanRuns {
        fn list<'a>(
            &'a self,
            _: &'a uuid::Uuid,
            _: Option<&'a str>,
        ) -> std::pin::Pin<
            Box<dyn Future<Output = Result<Page<ScanRunDto>, ScanRunError>> + Send + 'a>,
        > {
            todo!()
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
            cookie_value: expected_csrf_cookie_value,
        };
        let mock_register_feature = Arc::new(MockRegisterFeature);
        let mock_login_feature = Arc::new(MockLoginFeature);
        let mock_logout_feature = Arc::new(MockLogoutFeature);
        let mock_auth_feature = Arc::new(MockAuthFeature);
        let mock_token_feature = Arc::new(MockTokenFeature {
            return_value: expected_csrf_token_pair,
        });
        let mock_verify_csrf_feature = Arc::new(MockVerifyCsrfFeature);
        let mock_create_project = Arc::new(MockCreateProject);
        let mock_get_project = Arc::new(MockGetProject);
        let mock_list_projects = Arc::new(MockListProjects);
        let mock_create_scan = Arc::new(MockCreateScan);
        let mock_get_scan = Arc::new(MockGetScan);
        let mock_list_scans = Arc::new(MockListScans);
        let mock_get_scan_run = Arc::new(MockGetScanRun);
        let mock_list_scan_runs = Arc::new(MockListScanRuns);
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
        let mock_register_feature = Arc::new(MockRegisterFeature);
        let mock_login_feature = Arc::new(MockLoginFeature);
        let mock_logout_feature = Arc::new(MockLogoutFeature);
        let mock_auth_feature = Arc::new(MockAuthFeature);
        let mock_token_feature = Arc::new(MockTokenFeature {
            return_value: expected_csrf_token_pair,
        });
        let mock_verify_csrf_feature = Arc::new(MockVerifyCsrfFeature);
        let mock_create_project = Arc::new(MockCreateProject);
        let mock_get_project = Arc::new(MockGetProject);
        let mock_list_projects = Arc::new(MockListProjects);
        let mock_create_scan = Arc::new(MockCreateScan);
        let mock_get_scan = Arc::new(MockGetScan);
        let mock_list_scans = Arc::new(MockListScans);
        let mock_get_scan_run = Arc::new(MockGetScanRun);
        let mock_list_scan_runs = Arc::new(MockListScanRuns);
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
