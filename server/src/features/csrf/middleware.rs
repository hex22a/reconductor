use std::sync::Arc;

use axum::{
    Extension,
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use crate::{
    application::error::ServerError,
    constants::{CSRF_COOKIE_NAME, CSRF_HEADER},
    features::{
        csrf::{token::TokenFeature, verify::VerifyCsrfFeature},
        session::{auth::AuthFeature, model::UserSession},
        user::{login::LoginFeature, logout::LogoutFeature, register::RegisterFeature},
    },
    state::AppState,
};

pub(crate) async fn double_submit<R, L, O, T, A, C>(
    State(state): State<Arc<AppState<R, L, O, T, A, C>>>,
    headers: HeaderMap,
    jar: CookieJar,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    O: LogoutFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
    C: VerifyCsrfFeature + Clone + Send + Sync + 'static,
{
    let header_csrf_token = headers
        .get(CSRF_HEADER)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_owned())
        .ok_or(ServerError::Forbidden)?;
    let cookie_csrf_token = jar
        .get(CSRF_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or(ServerError::Forbidden)?;
    match state
        .verify_csrf_feature
        .verify_anonymous(cookie_csrf_token, header_csrf_token)
        .await
    {
        true => Ok(next.run(req).await),
        false => Err(ServerError::Forbidden),
    }
}

pub(crate) async fn session_based<R, L, O, T, A, C>(
    State(state): State<Arc<AppState<R, L, O, T, A, C>>>,
    headers: HeaderMap,
    jar: CookieJar,
    Extension(session): Extension<UserSession>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    O: LogoutFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
    C: VerifyCsrfFeature + Clone + Send + Sync + 'static,
{
    let header_csrf_token = headers
        .get(CSRF_HEADER)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_owned())
        .ok_or(ServerError::Forbidden)?;
    let cookie_csrf_token = jar
        .get(CSRF_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or(ServerError::Forbidden)?;
    let session_csrf_token = session.csrf_token;
    match state
        .verify_csrf_feature
        .verify_authorized(cookie_csrf_token, header_csrf_token, session_csrf_token)
        .await
    {
        true => Ok(next.run(req).await),
        false => Err(ServerError::Forbidden),
    }
}
