mod config;

use reconductor_messaging::{RabbitMqConfig, RabbitMqProvider};
use scanner::{AppError, DbConfig, Runner, Scanner, db};

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

    let rabbit_mq_config = RabbitMqConfig {
        username: config.rabbitmq_username,
        password: config.rabbitmq_password,
        host: config.rabbitmq_host,
        port: config.rabbitmq_port,
        vhost: config.rabbitmq_vhost,
    };
    let mq_provider = RabbitMqProvider::build(rabbit_mq_config)
        .await
        .expect("failed to connect to MQ");

    let app = Scanner::build(db, mq_provider);
    let _ = app.run().await;

    Ok(())
}
