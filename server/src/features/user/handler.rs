use crate::features::user::{model::RegisterUser, register::RegisterFeature};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    domain::error::ServerError, features::user::dto::RegisterUserRequest, state::AppState,
};

pub async fn register<R>(
    State(state): State<AppState<R>>,
    Json(req): Json<RegisterUserRequest>,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
{
    let user: RegisterUser = RegisterUser::try_from(req)?;
    state
        .register_feature
        .register(user.username, user.password)
        .await?;
    Ok((StatusCode::CREATED, ()))
}
