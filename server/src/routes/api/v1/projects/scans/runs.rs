use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    constants::API_SCAN_RUNS_ENDPOINT_V1,
    features::scan_run::handler::{get_scan_run, list},
    state::AppState,
};

mod hosts;

pub fn routes() -> Router<Arc<AppState>> {
    let host_routes = hosts::routes();
    let run_roures = Router::new()
        .route("/{run_id}", get(get_scan_run))
        .nest("/{run_id}", host_routes);

    let inner_routes = Router::new().route("/", get(list)).merge(run_roures);

    Router::new().nest(API_SCAN_RUNS_ENDPOINT_V1, inner_routes)
}
