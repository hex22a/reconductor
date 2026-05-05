use crate::features::user::{
    model::RegisterUser,
    register::{RegisterFeature, UserRegisterFeature},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    domain::error::ServerError, features::user::dto::RegisterUserRequest, state::AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterUserRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let user: RegisterUser = RegisterUser::try_from(req)?;
    let featue = UserRegisterFeature::new(state.password_service, state.user_repository);
    featue.register(&user.username, &user.password).await?;
    Ok((StatusCode::CREATED, ()))
}
