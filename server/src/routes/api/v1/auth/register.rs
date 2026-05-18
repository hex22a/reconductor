use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    constants::API_REGISTER_ENDPOINT_V1, features::user::handler::register, state::AppState,
};

pub(crate) fn routes(state: Arc<AppState>) -> Router {
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
            csrf::{
                error::CsrfError, model::CsrfTokenPair, token::TokenFeature,
                verify::VerifyCsrfFeature,
            },
            project::{
                create::CreateProjectFeature, dto::ProjectDto, error::ProjectError,
                list::ListProjectsFeature,
            },
            session::{auth::AuthFeature, error::SessionError, model::UserSession},
            user::{
                dto::UserInputRequest, error::UserError, login::LoginFeature,
                logout::LogoutFeature, model::AuthSession, register::RegisterFeature,
            },
        },
        state::AppState,
        transport::pagination::Page,
    };

    #[derive(Clone)]
    struct MockRegisterFeature;
    #[derive(Clone)]
    struct MockTokenFeature;
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
    struct MockListProjects;

    impl RegisterFeature for MockRegisterFeature {
        fn register(
            &self,
            _: String,
            _: String,
        ) -> std::pin::Pin<Box<dyn Future<Output = Result<(), UserError>> + Send + '_>> {
            Box::pin(async { Ok(()) })
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
            todo!()
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
        ) -> std::pin::Pin<Box<dyn Future<Output = Result<(), ProjectError>> + Send + '_>> {
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

    #[tokio::test]
    async fn test_register_post() {
        // Arrange
        let expected_username = "test".to_string();
        let expected_password = "password".to_string();
        let mock_register_feature = Arc::new(MockRegisterFeature);
        let mock_token_feature = Arc::new(MockTokenFeature);
        let mock_login_feature = Arc::new(MockLoginFeature);
        let mock_logout_feature = Arc::new(MockLogoutFeature);
        let mock_auth_feature = Arc::new(MockAuthFeature);
        let mock_verify_csrf_feature = Arc::new(MockVerifyCsrfFeature);
        let mock_create_project = Arc::new(MockCreateProject);
        let mock_list_projects = Arc::new(MockListProjects);
        let expected_app_state = Arc::new(AppState {
            register_feature: mock_register_feature,
            login_feature: mock_login_feature,
            logout_feature: mock_logout_feature,
            csrf_feature: mock_token_feature,
            auth_feature: mock_auth_feature,
            verify_csrf_feature: mock_verify_csrf_feature,
            create_project_feature: mock_create_project,
            list_projects_feature: mock_list_projects,
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
        let mock_register_feature = Arc::new(MockRegisterFeature);
        let mock_token_feature = Arc::new(MockTokenFeature);
        let mock_login_feature = Arc::new(MockLoginFeature);
        let mock_logout_feature = Arc::new(MockLogoutFeature);
        let mock_auth_feature = Arc::new(MockAuthFeature);
        let mock_verify_csrf_feature = Arc::new(MockVerifyCsrfFeature);
        let mock_create_project = Arc::new(MockCreateProject);
        let mock_list_projects = Arc::new(MockListProjects);
        let expected_app_state = Arc::new(AppState {
            register_feature: mock_register_feature,
            login_feature: mock_login_feature,
            logout_feature: mock_logout_feature,
            csrf_feature: mock_token_feature,
            auth_feature: mock_auth_feature,
            verify_csrf_feature: mock_verify_csrf_feature,
            create_project_feature: mock_create_project,
            list_projects_feature: mock_list_projects,
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
