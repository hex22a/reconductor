use sqlx::PgPool;

use crate::{
    features::scan::{
        poller::{PollerFeature, ScanPoller},
        repository::PgScanRepository,
    },
    infra::{
        message_queue::{RabbitMqProvider, publisher::MqPublisher},
        scheduler::Scheduler,
    },
};

pub mod application;
mod constants;
pub mod features;
pub mod infra;

pub struct ScanScheduler;

impl ScanScheduler {
    pub fn build(db: PgPool, mq: RabbitMqProvider, poll_interval_secs: u64) -> impl PollerFeature {
        ScanPoller::new(
            PgScanRepository { db },
            MqPublisher { provider: mq },
            Scheduler,
            poll_interval_secs,
        )
    }
}
