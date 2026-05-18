use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    application::error::ServerError,
    features::{
        project::{dto::CreateProjctRequest, model::CreateProjectInput},
        session::model::UserSession,
    },
    state::AppState,
};

pub(crate) async fn create(
    Extension(user_session): Extension<UserSession>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateProjctRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let project: CreateProjectInput = CreateProjectInput::try_from(req)?;
    Ok(StatusCode::CREATED)
}
