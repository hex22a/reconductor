use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};

use crate::{
    constants::{CSRF_COOKIE_NAME, USER_SESSION_COOKIE_NAME},
    domain::error::ServerError,
    features::{
        csrf::{dto::CsrfResponse, token::TokenFeature, verify::VerifyCsrfFeature},
        session::auth::AuthFeature,
        user::{login::LoginFeature, register::RegisterFeature},
    },
    state::AppState,
};

pub async fn handle<R, L, T, A, C>(
    State(state): State<Arc<AppState<R, L, T, A, C>>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
    C: VerifyCsrfFeature + Clone + Send + Sync + 'static,
{
    let session_cookie = jar
        .get(USER_SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string());
    let csrf_token_pair = state.csrf_feature.get_token(session_cookie).await?;
    let jar = jar.add(
        Cookie::build((CSRF_COOKIE_NAME, csrf_token_pair.cookie_value))
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
            csrf_token: csrf_token_pair.token,
        }),
    ))
}
