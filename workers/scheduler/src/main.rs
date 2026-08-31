mod config;

use scheduler::{
    ScanScheduler,
    application::error::AppError,
    features::scan::poller::PollerFeature,
    infra::{db, message_queue::RabbitMqProvider},
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let config = config::Config::from_env()?;

    let db = db::init_db(&config.database_url).await;
    let conn =
        lapin::Connection::connect(&config.rabbitmq_url, lapin::ConnectionProperties::default())
            .await?;
    info!("Connected to RabbitMQ");

    let publish_channel = conn.create_channel().await?;
    let mq_provider = RabbitMqProvider::build(publish_channel).await?;

    let scheduler = ScanScheduler::build(db, mq_provider, config.poll_interval_secs);

    scheduler.run().await?;

    Ok(())
}
