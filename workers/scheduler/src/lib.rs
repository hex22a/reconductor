use reconductor_messaging::{RabbitMqProvider, publisher::MqPublisher};
use reconductor_schedule::Scheduler;
use sqlx::PgPool;

use crate::features::scan::{
    poller::{PollerFeature, ScanPoller},
    repository::PgScanRepository,
};

pub mod application;
pub mod features;
pub mod infra;

pub struct ScanScheduler;

impl ScanScheduler {
    pub fn build(db: PgPool, mq: RabbitMqProvider, poll_interval_secs: u64) -> impl PollerFeature {
        ScanPoller::new(
            PgScanRepository { db },
            MqPublisher::new(mq),
            Scheduler,
            poll_interval_secs,
        )
    }
}
