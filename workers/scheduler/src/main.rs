mod config;

use scheduler::{
    ScanScheduler,
    application::error::AppError,
    features::scan::poller::PollerFeature,
    infra::{
        db::{self, DbConfig},
        message_queue::{RabbitMqConfig, RabbitMqProvider},
    },
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let config = config::Config::from_env()?;
    let db = db::init_db(DbConfig {
        username: config.db_username,
        password: config.db_password,
        host: config.db_host,
        port: config.db_port,
        db_name: config.db_name,
    })
    .await;
    let rabbitmq_uri = RabbitMqConfig {
        username: config.rabbitmq_username,
        password: config.rabbitmq_password,
        host: config.rabbitmq_host,
        port: config.rabbitmq_port,
        vhost: config.rabbitmq_vhost,
    }
    .uri();
    let conn =
        lapin::Connection::connect(&rabbitmq_uri, lapin::ConnectionProperties::default()).await?;
    info!("Connected to RabbitMQ");

    let publish_channel = conn.create_channel().await?;
    let mq_provider = RabbitMqProvider::build(publish_channel).await?;

    let scheduler = ScanScheduler::build(db, mq_provider, config.poll_interval_secs);

    scheduler.run().await?;

    Ok(())
}
