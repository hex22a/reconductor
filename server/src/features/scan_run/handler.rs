use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use reconductor_core::ScanId;
use uuid::Uuid;

use crate::{application::error::ServerError, domain::types::ScanRunId, state::AppState};

pub async fn get_scan_run(
    State(state): State<Arc<AppState>>,
    Path((_, _, run_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<impl IntoResponse, ServerError> {
    let scan_run = state
        .get_scan_run_feature
        .get(ScanRunId::new(run_id))
        .await?;
    Ok((StatusCode::OK, Json(scan_run)))
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    Path((_, scan_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, ServerError> {
    let scan_runs = state
        .list_scan_runs_feature
        .list(ScanId::new(scan_id), None)
        .await?;
    Ok((StatusCode::OK, Json(scan_runs)))
}
