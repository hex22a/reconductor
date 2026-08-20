use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    constants::API_PROJECT_SCANS_ENDPOINT_V1,
    features::scan::handler::{create, get_scan, list},
    state::AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    let inner_routes = Router::new()
        .route("/", post(create))
        .route("/", get(list))
        .route("/{scan_id}", get(get_scan));

    Router::new().nest(API_PROJECT_SCANS_ENDPOINT_V1, inner_routes)
}
