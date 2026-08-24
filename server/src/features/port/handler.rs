use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{application::error::ServerError, state::AppState};

pub async fn get_port(
    State(state): State<Arc<AppState>>,
    Path((_, _, _, _, port_id)): Path<(Uuid, Uuid, Uuid, Uuid, Uuid)>,
) -> Result<impl IntoResponse, ServerError> {
    let port = state.get_port_feature.get(port_id).await?;
    Ok((StatusCode::OK, Json(port)))
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    Path((_, _, _, host_id)): Path<(Uuid, Uuid, Uuid, Uuid)>,
) -> Result<impl IntoResponse, ServerError> {
    let ports = state.list_ports_feature.list(&host_id, None).await?;
    Ok((StatusCode::OK, Json(ports)))
}
