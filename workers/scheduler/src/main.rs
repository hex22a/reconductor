mod config;
mod db;
mod queue;
mod scheduler;

use db::scan::PgScanRepository;
use queue::publisher::RabbitMqPublisher;
use scheduler::Scheduler;
use scheduler::utils::SchedulerUtils;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let config = config::Config::from_env()?;

    let db = db::init_db(&config.database_url).await;
    let conn = lapin::Connection::connect(
        &config.rabbitmq_url,
        lapin::ConnectionProperties::default(),
    )
    .await?;
    info!("Connected to RabbitMQ");

    let publish_channel = conn.create_channel().await?;

    let scheduler = Scheduler::new(
        PgScanRepository { db },
        RabbitMqPublisher { channel: publish_channel },
        SchedulerUtils,
        config.poll_interval_secs,
    );

    scheduler.run().await?;

    Ok(())
}
