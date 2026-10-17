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
    domain::types::{HostId, ScanRunId},
    state::AppState,
};

pub async fn get_host(
    State(state): State<Arc<AppState>>,
    Path((_, _, _, host_id)): Path<(Uuid, Uuid, Uuid, Uuid)>,
) -> Result<impl IntoResponse, ServerError> {
    let host = state.get_host_feature.get(HostId::new(host_id)).await?;
    Ok((StatusCode::OK, Json(host)))
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    Path((_, _, run_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<impl IntoResponse, ServerError> {
    let hosts = state
        .list_hosts_feature
        .list(ScanRunId::new(run_id), None)
        .await?;
    Ok((StatusCode::OK, Json(hosts)))
}
