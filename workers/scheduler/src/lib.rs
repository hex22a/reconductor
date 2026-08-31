use sqlx::PgPool;

use crate::{
    features::scan::repository::PgScanRepository,
    infra::{
        message_queue::{provider::RabbitMqProvider, publisher::MqPublisher},
        scheduler::{ReconductorScheduler, Scheduler, utils::SchedulerUtils},
    },
};

pub mod application;
pub mod features;
pub mod infra;

pub struct ScanScheduler;

impl ScanScheduler {
    pub fn build(db: PgPool, mq: RabbitMqProvider, poll_interval_secs: u64) -> impl Scheduler {
        ReconductorScheduler::new(
            PgScanRepository { db },
            MqPublisher { provider: mq },
            SchedulerUtils,
            poll_interval_secs,
        )
    }
}
