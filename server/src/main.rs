use server::infra::message_queue::{RabbitMqConfig, RabbitMqProvider};
use server::infra::persistence::db::DbConfig;
use server::infra::persistence::kv::KvConfig;
use server::{AppError, Config, Reconductor};

use server::infra::persistence::{db, kv};

use tracing::info;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let config = Config::from_env()?;
    let db = db::init_db(DbConfig {
        username: config.db_username,
        password: config.db_password,
        host: config.db_host,
        port: config.db_port,
        db_name: config.db_name,
    })
    .await;
    let kv = kv::init_kv(KvConfig {
        username: config.kv_username,
        password: config.kv_password,
        host: config.kv_host,
        port: config.kv_port,
        database: config.kv_db,
    })
    .await;
    let rabbit_mq_uri = RabbitMqConfig {
        username: config.rabbitmq_username,
        password: config.rabbitmq_password,
        host: config.rabbitmq_host,
        port: config.rabbitmq_port,
        vhost: config.rabbitmq_vhost,
    }
    .uri();
    let conn =
        lapin::Connection::connect(&rabbit_mq_uri, lapin::ConnectionProperties::default()).await?;
    info!("Connected to RabbitMQ");
    let publish_channel = conn.create_channel().await?;
    let mq_provider = RabbitMqProvider::build(publish_channel)
        .await
        .expect("Can't declare a message queue");

    let app = Reconductor::build(db, kv, mq_provider, config.csrf_key, config.dashboard_url);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
