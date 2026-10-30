use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    application::error::ServerError,
    domain::types::ProjectId,
    features::{
        project::{dto::CreateProjctRequest, model::CreateProjectInput},
        session::model::UserSession,
    },
    state::AppState,
};

pub async fn create(
    Extension(user_session): Extension<UserSession>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateProjctRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let project: CreateProjectInput = CreateProjectInput::try_from(req)?;
    let owner_id = user_session.user_id;
    let project = state
        .create_project_feature
        .create(owner_id, project.name)
        .await?;
    Ok((StatusCode::CREATED, Json(project)))
}

pub async fn get_project(
    Extension(user_session): Extension<UserSession>,
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, ServerError> {
    let owner_id = user_session.user_id;
    let project = state
        .get_project_feature
        .get(ProjectId::new(project_id), owner_id)
        .await?;
    Ok((StatusCode::OK, Json(project)))
}

pub async fn list(
    Extension(user_session): Extension<UserSession>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ServerError> {
    let owner_id = user_session.user_id;
    let projects = state.list_projects_feature.list(owner_id, None).await?;
    Ok((StatusCode::OK, Json(projects)))
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use crate::{
        domain::{
            secret_types::{CsrfCookie, CsrfToken, SessionToken},
            types::{ProjectId, ProjectName, UserId, Username},
        },
        features::{
            csrf::{token::MockTokenFeature, verify::MockVerifyCsrfFeature},
            host::{get::MockGetHostFeature, list::MockListHostsFeature},
            port::{get::MockGetPortFeature, list::MockListPortsFeature},
            project::{
                create::MockCreateProjectFeature, dto::ProjectDto, get::MockGetProjectFeature,
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
        test_helpers::body_json,
    };

    use super::*;

    #[tokio::test]
    async fn test_create_project() {
        // Arrange
        let expected_session_token = SessionToken::new("session_id");
        let expected_user_id = UserId::new(Uuid::now_v7());
        let expected_username = Username::new("test");
        let expected_csrf_token = CsrfToken::new("token");
        let expected_csrf_cookie = CsrfCookie::new("cookie");
        let expected_user_session = UserSession {
            token: expected_session_token,
            user_id: expected_user_id,
            username: expected_username,
            csrf_token: expected_csrf_token,
            csrf_cookie: expected_csrf_cookie,
        };

        let expected_project_name_string = String::from("test_project");
        let expected_request = CreateProjctRequest {
            name: expected_project_name_string.clone(),
        };

        let expected_project_id = ProjectId::new(Uuid::now_v7());
        let expected_project_name = ProjectName::new(expected_project_name_string);
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_response = ProjectDto {
            id: expected_project_id,
            name: expected_project_name,
            created_at: expected_created_at,
        };
        let expected_project_dto = expected_response.clone();

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
        let mock_auth_feature = Arc::new(MockAuthFeature::new());
        let mock_verify_csrf_feature = Arc::new(MockVerifyCsrfFeature::new());
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

        // Act
        let actual_response = create(
            Extension(expected_user_session),
            State(expected_app_state),
            Json(expected_request),
        )
        .await
        .unwrap()
        .into_response();

        // Assert
        assert_eq!(actual_response.status(), StatusCode::CREATED);
        assert_eq!(
            body_json::<ProjectDto>(actual_response).await,
            expected_response
        );
    }
}
