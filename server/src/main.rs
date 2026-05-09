use std::sync::{Arc, Mutex};

use crate::{
    constants::REDIS_POOL_SIZE,
    features::{csrf::token::CsrfTokenFeature, user::register::UserRegisterFeature},
    infra::{
        csrf::AesGcmCsrfService,
        password::Argon2Service,
        persistence::{
            db::{self, user::PgUserRepository},
            kv::{FredKvProvider, csrf::CsrfStore, session::SessionStore},
        },
        random::OsRngService,
    },
};
use axum::Router;
use rand::rngs::SysRng;

mod config;
mod constants;
mod domain;
mod features;
mod infra;
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
    let kv = Arc::new(FredKvProvider::new(config.redis_url, REDIS_POOL_SIZE).await?);
    let user_repository = PgUserRepository { db };
    let session_repository = SessionStore::new(Arc::clone(&kv));
    let csrf_repository = CsrfStore::new(Arc::clone(&kv));
    let password_service = Argon2Service;
    let os_rng_serivce = OsRngService::new(Arc::new(Mutex::new(SysRng)));
    let csrf_service = AesGcmCsrfService::new(os_rng_serivce, config.csrf_key);
    let register_feature = UserRegisterFeature::new(password_service, user_repository);
    let csrf_feature = CsrfTokenFeature::new(session_repository, csrf_repository, csrf_service);
    let app_state = Arc::new(AppState {
        register_feature,
        csrf_feature,
    });

    let app = Router::new()
        .merge(v1::health::routes())
        .merge(v1::auth::register::routes(Arc::clone(&app_state)))
        .merge(v1::csrf::routes(Arc::clone(&app_state)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
