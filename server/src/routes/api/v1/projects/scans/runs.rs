use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    constants::API_SCAN_RUNS_ENDPOINT_V1,
    features::scan_run::handler::{get_scan_run, list},
    state::AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    let inner_routes = Router::new()
        .route("/", get(list))
        .route("/{run_id}", get(get_scan_run));

    Router::new().nest(API_SCAN_RUNS_ENDPOINT_V1, inner_routes)
}
