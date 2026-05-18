use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    application::error::ServerError,
    features::{
        csrf::{token::TokenFeature, verify::VerifyCsrfFeature},
        project::{dto::CreateProjctRequest, model::CreateProjectInput},
        session::{auth::AuthFeature, model::UserSession},
        user::{login::LoginFeature, logout::LogoutFeature, register::RegisterFeature},
    },
    state::AppState,
};

pub(crate) async fn create<R, L, O, T, A, C>(
    Extension(user_session): Extension<UserSession>,
    State(state): State<Arc<AppState<R, L, O, T, A, C>>>,
    Json(req): Json<CreateProjctRequest>,
) -> Result<impl IntoResponse, ServerError>
where
    R: RegisterFeature + Clone + Send + Sync + 'static,
    L: LoginFeature + Clone + Send + Sync + 'static,
    O: LogoutFeature + Clone + Send + Sync + 'static,
    T: TokenFeature + Clone + Send + Sync + 'static,
    A: AuthFeature + Clone + Send + Sync + 'static,
    C: VerifyCsrfFeature + Clone + Send + Sync + 'static,
{
    let project: CreateProjectInput = CreateProjectInput::try_from(req)?;
    Ok(StatusCode::CREATED)
}
