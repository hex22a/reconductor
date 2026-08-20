use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    constants::API_PROJECT_SCANS_ENDPOINT_V1, features::scan::handler::create, state::AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    let inner_routes = Router::new().route("/", post(create));

    Router::new().nest(API_PROJECT_SCANS_ENDPOINT_V1, inner_routes)
}
