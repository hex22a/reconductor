use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use secrecy::ExposeSecret;

use crate::{
    application::error::ServerError,
    constants::{CSRF_COOKIE_NAME, USER_SESSION_COOKIE_NAME},
    domain::secret_types::SessionToken,
    features::csrf::dto::CsrfResponse,
    state::AppState,
};

pub async fn handle(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ServerError> {
    let session_cookie = jar
        .get(USER_SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string());
    let csrf_token_pair = state
        .csrf_feature
        .get_token(session_cookie.map(SessionToken::new))
        .await?;
    let jar = jar.add(
        Cookie::build((
            CSRF_COOKIE_NAME,
            csrf_token_pair
                .cookie_value
                .into_inner()
                .expose_secret()
                .to_owned(),
        ))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .build(),
    );
    Ok((
        StatusCode::OK,
        jar,
        Json(CsrfResponse {
            csrf_token: csrf_token_pair
                .token
                .into_inner()
                .expose_secret()
                .to_owned(),
        }),
    ))
}

#[cfg(test)]
mod tests {
    use crate::{
        CsrfCookie,
        domain::secret_types::CsrfToken,
        features::{
            csrf::{
                error::CsrfError, model::CsrfTokenPair, token::MockTokenFeature,
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
        test_helpers::{body_json, set_cookie},
    };

    use super::*;

    #[tokio::test]
    async fn test_get_csrf_happy_path() {
        // Arrange
        let expected_csrf_token = CsrfToken::new("token");
        let expected_token_pair = CsrfTokenPair {
            token: expected_csrf_token.clone(),
            cookie_value: CsrfCookie::new("cookie"),
        };
        let expected_cookie_value = expected_token_pair
            .clone()
            .cookie_value
            .into_inner()
            .expose_secret()
            .to_owned();

        let mut mock_token_feature = MockTokenFeature::new();
        mock_token_feature.expect_get_token().returning(move |_| {
            let token_pair = expected_token_pair.clone();
            Box::pin(async { Ok(token_pair) })
        });
        let mock_register_feature = Arc::new(MockRegisterFeature::new());
        let mock_token_feature = Arc::new(mock_token_feature);
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

        let expected_cookie_jar = CookieJar::new();
        let expected_cookie = Cookie::build((CSRF_COOKIE_NAME, expected_cookie_value))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .path("/")
            .build();
        let expected_response = CsrfResponse {
            csrf_token: expected_csrf_token.into_inner().expose_secret().to_owned(),
        };

        // Act
        let actual_response = handle(State(expected_app_state), expected_cookie_jar)
            .await
            .unwrap()
            .into_response();

        // Assert
        assert_eq!(actual_response.status(), StatusCode::OK);
        assert_eq!(
            set_cookie(&actual_response, CSRF_COOKIE_NAME),
            expected_cookie
        );
        assert_eq!(
            body_json::<CsrfResponse>(actual_response).await,
            expected_response
        );
    }

    #[tokio::test]
    async fn test_get_csrf_error_getting_token() {
        // Arrange
        let mut mock_token_feature = MockTokenFeature::new();
        mock_token_feature
            .expect_get_token()
            .returning(|_| Box::pin(async { Err(CsrfError::AnonymousNotCreated) }));
        let mock_register_feature = Arc::new(MockRegisterFeature::new());
        let mock_token_feature = Arc::new(mock_token_feature);
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

        let expected_cookie_jar = CookieJar::new();

        // Act
        let actual_response = handle(State(expected_app_state), expected_cookie_jar).await;

        // Assert
        assert!(matches!(actual_response, Err(ServerError::Internal)));
    }
}
