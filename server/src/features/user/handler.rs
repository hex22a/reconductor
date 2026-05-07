use crate::{
    features::user::{
        model::RegisterUser,
        register::{RegisterFeature, UserRegisterFeature},
    },
    infra::password::PasswordService,
    persistence::db::user::UserRepository,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    domain::error::ServerError, features::user::dto::RegisterUserRequest, state::AppState,
};

pub async fn register<P, U>(
    State(state): State<AppState<P, U>>,
    Json(req): Json<RegisterUserRequest>,
) -> Result<impl IntoResponse, ServerError>
where
    P: PasswordService + Clone + Send + Sync + 'static,
    U: UserRepository + Clone + Send + Sync + 'static,
{
    let user: RegisterUser = RegisterUser::try_from(req)?;
    let featue = UserRegisterFeature::new(state.password_service, state.user_repository);
    featue.register(&user.username, &user.password).await?;
    Ok((StatusCode::CREATED, ()))
}
