use std::sync::Arc;

use crate::{
    constants::{CSRF_COOKIE_NAME, USER_SESSION_COOKIE_NAME},
    features::{
        csrf::{token::TokenFeature, verify::VerifyCsrfFeature},
        session::{auth::AuthFeature, model::UserSession},
        user::{
            dto::{LoginResponse, MeResponse},
            login::LoginFeature,
            model::UserInput,
            register::RegisterFeature,
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

pub async fn register<R, L, T, A, C>(
    State(state): State<Arc<AppState<R, L, T, A, C>>>,
    Json(req): Json<UserInputRequest>,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
    C: VerifyCsrfFeature + Clone + Send + Sync + 'static,
{
    let user: UserInput = UserInput::try_from(req)?;
    state
        .register_feature
        .register(user.username, user.password)
        .await?;
    Ok((StatusCode::CREATED, ()))
}

pub async fn login<R, L, T, A, C>(
    jar: CookieJar,
    State(state): State<Arc<AppState<R, L, T, A, C>>>,
    Json(req): Json<UserInputRequest>,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
    C: VerifyCsrfFeature + Clone + Send + Sync + 'static,
{
    let user: UserInput = UserInput::try_from(req)?;
    let anonymous_csrf_token = jar
        .get(CSRF_COOKIE_NAME)
        .map(|c| c.to_string())
        .ok_or(ServerError::Forbidden)?;
    let auth_session = state
        .login_feature
        .login(user.username, user.password, anonymous_csrf_token)
        .await?;
    let jar = jar
        .add(
            Cookie::build((USER_SESSION_COOKIE_NAME, auth_session.session_id))
                .http_only(true)
                .secure(true)
                .same_site(SameSite::Lax)
                .path("/")
                .build(),
        )
        .add(
            Cookie::build((CSRF_COOKIE_NAME, auth_session.csrf_cookie))
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
            csrf_token: auth_session.csrf_token,
        }),
    ))
}

pub async fn me(Extension(user_session): Extension<UserSession>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(MeResponse {
            username: user_session.username,
        }),
    )
}
