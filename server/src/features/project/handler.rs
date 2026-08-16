use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

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
    let project = state
        .create_project_feature
        .create(owner_id, project.name)
        .await?;
    Ok((StatusCode::CREATED, Json(project)))
}

pub(crate) async fn get_project(
    Extension(user_session): Extension<UserSession>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ServerError> {
    let owner_id = user_session.user_id;
    let project = state.get_project_feature.get(id, owner_id).await?;
    Ok((StatusCode::OK, Json(project)))
}

pub(crate) async fn list(
    Extension(user_session): Extension<UserSession>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ServerError> {
    let owner_id = user_session.user_id;
    let projects = state.list_projects_feature.list(&owner_id, None).await?;
    Ok((StatusCode::OK, Json(projects)))
}
