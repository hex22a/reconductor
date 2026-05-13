use std::sync::{Arc, Mutex};

use crate::{
    constants::{CSRF_HEADER, REDIS_POOL_SIZE},
    features::{
        csrf::{token::CsrfTokenFeature, verify::StatefulCsrfVerifier},
        session::auth::UserAuthFeature,
        user::{
            login::UserLoginFeature, register::UserRegisterFeature, repository::PgUserRepository,
        },
    },
    infra::{
        csrf::AesGcmCsrfService,
        password::Argon2Service,
        persistence::{
            db,
            kv::{FredKvProvider, csrf::CsrfStore, session::SessionStore},
        },
        random::OsRngService,
    },
};
use axum::{
    Router,
    http::{HeaderName, Method, header::CONTENT_TYPE},
};
use rand::rngs::SysRng;
use tower_http::cors::CorsLayer;

mod application;
mod config;
mod constants;
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
    let user_repository = Arc::new(PgUserRepository { db });
    let session_repository = Arc::new(SessionStore::new(Arc::clone(&kv)));
    let csrf_repository = Arc::new(CsrfStore::new(Arc::clone(&kv)));
    let password_service = Arc::new(Argon2Service);
    let os_rng_serivce = Arc::new(OsRngService::new(Arc::new(Mutex::new(SysRng))));
    let csrf_service = Arc::new(AesGcmCsrfService::new(
        Arc::clone(&os_rng_serivce),
        config.csrf_key,
    ));
    let register_feature =
        UserRegisterFeature::new(Arc::clone(&password_service), Arc::clone(&user_repository));
    let login_feature = UserLoginFeature::new(
        Arc::clone(&user_repository),
        Arc::clone(&session_repository),
        Arc::clone(&csrf_repository),
        Arc::clone(&csrf_service),
        Arc::clone(&password_service),
        Arc::clone(&os_rng_serivce),
    );
    let csrf_feature = CsrfTokenFeature::new(
        Arc::clone(&session_repository),
        Arc::clone(&csrf_repository),
        Arc::clone(&csrf_service),
    );
    let auth_feature = UserAuthFeature::new(Arc::clone(&session_repository));
    let verify_csrf_feature =
        StatefulCsrfVerifier::new(Arc::clone(&csrf_service), Arc::clone(&csrf_repository));
    let app_state = Arc::new(AppState {
        register_feature,
        login_feature,
        csrf_feature,
        auth_feature,
        verify_csrf_feature,
    });

    let cors = CorsLayer::new()
        .allow_origin(config.dashboard_url)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE, CSRF_HEADER.parse::<HeaderName>().unwrap()])
        .allow_credentials(true);

    let app = Router::new()
        .merge(v1::health::routes())
        .merge(v1::auth::register::routes(Arc::clone(&app_state)))
        .merge(v1::auth::login::routes(Arc::clone(&app_state)))
        .merge(v1::me::routes(Arc::clone(&app_state)))
        .merge(v1::csrf::routes(Arc::clone(&app_state)))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
