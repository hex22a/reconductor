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

mod runs;

pub fn routes() -> Router<Arc<AppState>> {
    let scan_run_routes = runs::routes();
    let scan_routes = Router::new()
        .route("/{scan_id}", get(get_scan))
        .nest("/{scan_id}", scan_run_routes);
    let inner_routes = Router::new()
        .route("/", post(create))
        .route("/", get(list))
        .merge(scan_routes);

    Router::new().nest(API_PROJECT_SCANS_ENDPOINT_V1, inner_routes)
}
