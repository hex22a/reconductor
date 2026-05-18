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

#[axum::debug_handler]
pub(crate) async fn create(
    Extension(user_session): Extension<UserSession>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateProjctRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let project: CreateProjectInput = CreateProjectInput::try_from(req)?;
    let owner_id = user_session.user_id;
    state
        .create_project_feature
        .create(owner_id, project.name)
        .await?;
    Ok(StatusCode::CREATED)
}
