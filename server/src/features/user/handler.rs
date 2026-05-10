use std::sync::Arc;

use crate::features::{
    csrf::token::TokenFeature,
    user::{dto::LoginResponse, login::LoginFeature, model::UserInput, register::RegisterFeature},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{domain::error::ServerError, features::user::dto::UserInputRequest, state::AppState};

pub async fn register<R, L, T>(
    State(state): State<Arc<AppState<R, L, T>>>,
    Json(req): Json<UserInputRequest>,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
{
    let user: UserInput = UserInput::try_from(req)?;
    state
        .register_feature
        .register(user.username, user.password)
        .await?;
    Ok((StatusCode::CREATED, ()))
}

pub async fn login<R, L, T>(
    jar: CookieJar,
    State(state): State<Arc<AppState<R, L, T>>>,
    Json(req): Json<UserInputRequest>,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
{
    let user: UserInput = UserInput::try_from(req)?;
    let auth_session = state
        .login_feature
        .login(user.username, user.password)
        .await?;
    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            csrf_token: auth_session.csrf_token,
        }),
    ))
}
