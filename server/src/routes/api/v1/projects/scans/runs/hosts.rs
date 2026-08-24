use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    constants::API_HOSTS_ENDPOINT_V1,
    features::host::handler::{get_host, list},
    state::AppState,
};

mod ports;

pub fn routes() -> Router<Arc<AppState>> {
    let port_routes = ports::routes();
    let host_routes = Router::new()
        .route("/{host_id}", get(get_host))
        .nest("/{host_id}", port_routes);
    let inner_routes = Router::new().route("/", get(list)).merge(host_routes);

    Router::new().nest(API_HOSTS_ENDPOINT_V1, inner_routes)
}
