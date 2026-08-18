use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};

use crate::{
    application::error::ServerError,
    constants::{CSRF_COOKIE_NAME, USER_SESSION_COOKIE_NAME},
    features::csrf::dto::CsrfResponse,
    state::AppState,
};

pub async fn handle(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ServerError> {
    let session_cookie = jar.get(USER_SESSION_COOKIE_NAME).map(|c| c.value());
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
