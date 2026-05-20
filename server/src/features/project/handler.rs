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
    let owner_id = user_session.user_id;
    state
        .create_project_feature
        .create(owner_id, project.name)
        .await?;
    Ok(StatusCode::CREATED)
}

pub(crate) async fn list(
    Extension(user_session): Extension<UserSession>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ServerError> {
    let owner_id = user_session.user_id;
    let projects = state.list_projects_feature.list(&owner_id, None).await?;
    Ok((StatusCode::OK, Json(projects)))
}
