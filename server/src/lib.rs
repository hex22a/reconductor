use std::sync::{Arc, Mutex};

use axum::Router;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderName, Method};
use rand::rngs::SysRng;
use sqlx::PgPool;

pub use infra::persistence::kv::FredKvProvider;

use features::csrf::repository::CsrfStore;
use features::project::repository::PgProjectRepository;
use features::session::repository::SessionStore;
use features::user::repository::PgUserRepository;
use tower_http::cors::CorsLayer;

pub use config::Config;
use constants::CSRF_HEADER;
use features::csrf::token::CsrfTokenFeature;
use features::csrf::verify::StatefulCsrfVerifier;
use features::project::get::GetProject;
use features::scan::create::CreateScan;
use features::scan::repository::PgScanRespository;
use features::session::auth::UserAuthFeature;
use features::user::login::UserLoginFeature;
use features::user::logout::UserLogoutFeature;
use features::user::register::UserRegisterFeature;
use infra::csrf::AesGcmCsrfService;
use infra::password::Argon2Service;
use infra::random::OsRngService;
use infra::scheduler::Scheduler;
use routes::api::v1;
use state::AppState;

use features::project::create::CreateProject;
use features::project::list::ListProjects;

use features::scan::list::ListScans;

use crate::features::host::get::GetHost;
use crate::features::host::list::ListHosts;
use crate::features::host::repository::PgHostRepository;
use crate::features::scan::get::GetScan;
use crate::features::scan_run::get::GetScanRun;
use crate::features::scan_run::list::ListScanRuns;
use crate::features::scan_run::repository::PgScanRunRepository;

mod application;
mod config;
mod constants;
mod domain;
pub mod features;
pub mod infra;
mod routes;
mod state;
mod transport;

pub struct Reconductor;

impl Reconductor {
    pub fn build(db: PgPool, kv: FredKvProvider, config: Config) -> Router {
        let db = Arc::new(db);
        let kv = Arc::new(kv);

        let user_repository = Arc::new(PgUserRepository::new(Arc::clone(&db)));
        let session_repository = Arc::new(SessionStore::new(Arc::clone(&kv)));
        let csrf_repository = Arc::new(CsrfStore::new(Arc::clone(&kv)));
        let project_repository = Arc::new(PgProjectRepository::new(Arc::clone(&db)));
        let scan_repository = Arc::new(PgScanRespository::new(Arc::clone(&db)));
        let scan_run_repository = Arc::new(PgScanRunRepository::new(Arc::clone(&db)));
        let host_repository = Arc::new(PgHostRepository::new(Arc::clone(&db)));
        let scheduler_service = Arc::new(Scheduler);
        let password_service = Arc::new(Argon2Service);
        let os_rng_serivce = Arc::new(OsRngService::new(Arc::new(Mutex::new(SysRng))));
        let csrf_service = Arc::new(AesGcmCsrfService::new(
            Arc::clone(&os_rng_serivce),
            config.csrf_key,
        ));
        let register_feature = Arc::new(UserRegisterFeature::new(
            Arc::clone(&password_service),
            Arc::clone(&user_repository),
        ));
        let login_feature = Arc::new(UserLoginFeature::new(
            Arc::clone(&user_repository),
            Arc::clone(&session_repository),
            Arc::clone(&csrf_repository),
            Arc::clone(&csrf_service),
            Arc::clone(&password_service),
            Arc::clone(&os_rng_serivce),
        ));
        let logout_feature = Arc::new(UserLogoutFeature::new(Arc::clone(&session_repository)));
        let csrf_feature = Arc::new(CsrfTokenFeature::new(
            Arc::clone(&session_repository),
            Arc::clone(&csrf_repository),
            Arc::clone(&csrf_service),
        ));
        let auth_feature = Arc::new(UserAuthFeature::new(Arc::clone(&session_repository)));
        let verify_csrf_feature = Arc::new(StatefulCsrfVerifier::new(
            Arc::clone(&csrf_service),
            Arc::clone(&csrf_repository),
        ));
        let create_project_feature = Arc::new(CreateProject::new(Arc::clone(&project_repository)));
        let get_project_feature = Arc::new(GetProject::new(Arc::clone(&project_repository)));
        let list_projects_feature = Arc::new(ListProjects::new(Arc::clone(&project_repository)));
        let create_scan_feature = Arc::new(CreateScan::new(
            Arc::clone(&scan_repository),
            Arc::clone(&scheduler_service),
        ));
        let get_scan_feature = Arc::new(GetScan::new(Arc::clone(&scan_repository)));
        let list_scans_feature = Arc::new(ListScans::new(Arc::clone(&scan_repository)));
        let get_scan_run_feature = Arc::new(GetScanRun::new(Arc::clone(&scan_run_repository)));
        let list_scan_runs_feature = Arc::new(ListScanRuns::new(Arc::clone(&scan_run_repository)));
        let get_host_feature = Arc::new(GetHost::new(Arc::clone(&host_repository)));
        let list_hosts_feature = Arc::new(ListHosts::new(Arc::clone(&host_repository)));

        let app_state = Arc::new(AppState {
            register_feature,
            login_feature,
            logout_feature,
            csrf_feature,
            auth_feature,
            verify_csrf_feature,
            create_project_feature,
            get_project_feature,
            list_projects_feature,
            create_scan_feature,
            get_scan_feature,
            list_scans_feature,
            get_scan_run_feature,
            list_scan_runs_feature,
            get_host_feature,
            list_hosts_feature,
        });

        let cors = CorsLayer::new()
            .allow_origin(config.dashboard_url)
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([CONTENT_TYPE, CSRF_HEADER.parse::<HeaderName>().unwrap()])
            .allow_credentials(true);

        Router::new()
            .merge(v1::health::routes())
            .merge(v1::auth::register::routes(Arc::clone(&app_state)))
            .merge(v1::auth::login::routes(Arc::clone(&app_state)))
            .merge(v1::auth::logout::routes(Arc::clone(&app_state)))
            .merge(v1::me::routes(Arc::clone(&app_state)))
            .merge(v1::csrf::routes(Arc::clone(&app_state)))
            .merge(v1::projects::routes(Arc::clone(&app_state)))
            .layer(cors)
    }
}
