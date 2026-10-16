use std::sync::Arc;

use crate::{
    constants::{CSRF_COOKIE_NAME, USER_SESSION_COOKIE_NAME},
    domain::types::{CsrfToken, SessionToken},
    features::{
        session::model::UserSession,
        user::{
            dto::{LoginResponse, MeResponse},
            model::UserInput,
        },
    },
};
use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};

use crate::{
    application::error::ServerError, features::user::dto::UserInputRequest, state::AppState,
};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<UserInputRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let user: UserInput = UserInput::try_from(req)?;
    state
        .register_feature
        .register(user.username, user.password)
        .await?;
    Ok((StatusCode::CREATED, ()))
}

pub async fn login(
    jar: CookieJar,
    State(state): State<Arc<AppState>>,
    Json(req): Json<UserInputRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let user: UserInput = UserInput::try_from(req)?;
    let anonymous_csrf_token = jar
        .get(CSRF_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or(ServerError::Forbidden)?;
    let auth_session = state
        .login_feature
        .login(
            user.username,
            user.password,
            CsrfToken::new(anonymous_csrf_token),
        )
        .await?;
    let jar = jar
        .add(
            Cookie::build((
                USER_SESSION_COOKIE_NAME,
                auth_session.session_id.into_inner(),
            ))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .path("/")
            .build(),
        )
        .add(
            Cookie::build((CSRF_COOKIE_NAME, auth_session.csrf_cookie.into_inner()))
                .http_only(true)
                .secure(true)
                .same_site(SameSite::Lax)
                .path("/")
                .build(),
        );
    Ok((
        StatusCode::OK,
        jar,
        Json(LoginResponse {
            csrf_token: auth_session.csrf_token.into_inner(),
        }),
    ))
}

pub async fn me(Extension(user_session): Extension<UserSession>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(MeResponse {
            username: user_session.username.into_inner(),
        }),
    )
}

pub async fn logout(
    jar: CookieJar,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ServerError> {
    let session_id = jar
        .get(USER_SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or(ServerError::Unauthorized)?;
    state
        .logout_feature
        .logout(SessionToken::new(session_id))
        .await?;
    let jar = jar.remove(
        Cookie::build(USER_SESSION_COOKIE_NAME)
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .path("/")
            .build(),
    );
    Ok((StatusCode::NO_CONTENT, jar))
}
