use std::sync::Arc;

use crate::{
    constants::USER_SESSION_COOKIE_NAME,
    features::{
        csrf::token::TokenFeature,
        session::auth::AuthFeature,
        user::{
            dto::LoginResponse, login::LoginFeature, model::UserInput, register::RegisterFeature,
        },
    },
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{CookieJar, cookie::Cookie};

use crate::{domain::error::ServerError, features::user::dto::UserInputRequest, state::AppState};

pub async fn register<R, L, T, A>(
    State(state): State<Arc<AppState<R, L, T, A>>>,
    Json(req): Json<UserInputRequest>,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
{
    let user: UserInput = UserInput::try_from(req)?;
    state
        .register_feature
        .register(user.username, user.password)
        .await?;
    Ok((StatusCode::CREATED, ()))
}

pub async fn login<R, L, T, A>(
    jar: CookieJar,
    State(state): State<Arc<AppState<R, L, T, A>>>,
    Json(req): Json<UserInputRequest>,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
{
    let user: UserInput = UserInput::try_from(req)?;
    let auth_session = state
        .login_feature
        .login(user.username, user.password)
        .await?;
    let jar = jar.add(
        Cookie::build((USER_SESSION_COOKIE_NAME, auth_session.session_id))
            .http_only(true)
            .secure(true)
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
