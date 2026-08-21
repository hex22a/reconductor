use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{application::error::ServerError, state::AppState};

pub async fn get_host(
    State(state): State<Arc<AppState>>,
    Path((_, _, _, host_id)): Path<(Uuid, Uuid, Uuid, Uuid)>,
) -> Result<impl IntoResponse, ServerError> {
    let host = state.get_host_feature.get(host_id).await?;
    Ok((StatusCode::OK, Json(host)))
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    Path((_, _, run_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<impl IntoResponse, ServerError> {
    let hosts = state.list_scan_runs_feature.list(&run_id, None).await?;
    Ok((StatusCode::OK, Json(hosts)))
}
