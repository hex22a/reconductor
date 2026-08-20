use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    application::error::ServerError,
    features::scan::{dto::CreateScanRequest, model::CreateScanInput},
    state::AppState,
};

pub async fn create(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<Uuid>,
    Json(req): Json<CreateScanRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let scan: CreateScanInput = CreateScanInput::try_from(req)?;
    let CreateScanInput { target, schedule } = scan;
    let scan = state
        .create_scan_feature
        .create(project_id, target, schedule)
        .await?;
    Ok((StatusCode::CREATED, Json(scan)))
}

pub async fn get_scan(
    State(state): State<Arc<AppState>>,
    Path(scan_id): Path<Uuid>,
) -> Result<impl IntoResponse, ServerError> {
    let scan = state.get_scan_feature.get(scan_id).await?;
    Ok((StatusCode::OK, Json(scan)))
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, ServerError> {
    let scans = state.list_scans_feature.list(&project_id, None).await?;
    Ok((StatusCode::OK, Json(scans)))
}
