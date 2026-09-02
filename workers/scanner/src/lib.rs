use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    application::run::ApplicationRunner,
    features::{
        scan::{repository::PgScanRepository, update::UpdateScan},
        scan_result::{add::AddScanResult, repository::PgScanResultRepository},
    },
    infra::{message_queue::consumer::MqConsumer, nmap::NmapRunner},
};

mod application;
pub mod constants;
mod domain;
pub mod features;
mod infra;

pub use application::run::Runner;
pub use infra::db;
pub use infra::message_queue::RabbitMqProvider;

pub struct Scanner;

impl Scanner {
    pub fn build(db: PgPool, mq: RabbitMqProvider) -> impl Runner {
        let db = Arc::new(db);

        let scan_repository = PgScanRepository::new(Arc::clone(&db));
        let scan_result_repository = PgScanResultRepository::new(Arc::clone(&db));
        let update_scan_feature = UpdateScan::new(scan_repository);
        let add_scan_result_feature = AddScanResult::new(scan_result_repository);
        let consumer = MqConsumer::new(mq);
        ApplicationRunner::new(
            consumer,
            NmapRunner,
            update_scan_feature,
            add_scan_result_feature,
        )
    }
}
