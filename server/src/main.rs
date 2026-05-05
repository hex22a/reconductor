use crate::{
    infra::password::Argon2Service,
    persistence::db::{self, user::PgUserRepository},
};
use axum::Router;

mod config;
mod constants;
mod controllers;
mod domain;
mod features;
mod infra;
mod persistence;
mod routes;
mod state;
mod transport;

use crate::{routes::api::v1, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let config = config::Config::from_env()?;
    let db = db::init_db(&config.database_url).await;
    let user_repository = PgUserRepository { db };
    let password_service = Argon2Service;
    let app_state = AppState {
        user_repository,
        password_service,
    };

    let app = Router::new()
        .merge(v1::health::routes())
        .merge(v1::auth::register::routes(app_state));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
