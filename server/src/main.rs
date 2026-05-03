use axum::Router;

mod constants;
mod controllers;
mod persistence;
mod routes;
mod services;

use crate::routes::api::v1;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().merge(v1::health::routes());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
