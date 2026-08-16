use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    application::error::ServerError,
    features::scan::{dto::CreateScanRequest, model::CreateScanInput},
    state::AppState,
};

pub(crate) async fn create(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateScanRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let scan: CreateScanInput = CreateScanInput::try_from(req)?;
    let CreateScanInput {
        project_id,
        target,
        schedule,
    } = scan;
    let scan = state
        .create_scan_feature
        .create(project_id, target, schedule)
        .await?;
    Ok((StatusCode::CREATED, Json(scan)))
}
