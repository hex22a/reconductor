mod config;

use scanner::{AppError, DbConfig, RabbitMqConfig, RabbitMqProvider, Runner, Scanner, db};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = config::Config::from_env()?;
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

    let consume_channel = conn.create_channel().await?;
    let db = db::init_db(DbConfig {
        username: config.db_username,
        password: config.db_password,
        host: config.db_host,
        port: config.db_port,
        db_name: config.db_name,
    })
    .await;
    let mq_provider = RabbitMqProvider::build(consume_channel)
        .await
        .expect("Can't declare a message queue");

    let app = Scanner::build(db, mq_provider);
    let _ = app.run().await;

    Ok(())
}
