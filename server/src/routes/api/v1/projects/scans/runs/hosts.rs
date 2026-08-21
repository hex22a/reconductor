use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    constants::API_HOSTS_ENDPOINT_V1,
    features::host::handler::{get_host, list},
    state::AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    let inner_routes = Router::new()
        .route("/", get(list))
        .route("/{host_id}", get(get_host));

    Router::new().nest(API_HOSTS_ENDPOINT_V1, inner_routes)
}
