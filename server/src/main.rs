use server::infra::message_queue::RabbitMqProvider;
use server::{Config, Reconductor};

use server::infra::persistence::{db, kv};

use constants::REDIS_POOL_SIZE;
use tracing::info;

mod constants;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let config = Config::from_env()?;
    let db = db::init_db(&config.database_url).await;
    let kv = kv::init_kv(&config.redis_url, REDIS_POOL_SIZE).await;
    let conn =
        lapin::Connection::connect(&config.rabbitmq_url, lapin::ConnectionProperties::default())
            .await?;
    info!("Connected to RabbitMQ");
    let publish_channel = conn.create_channel().await?;
    let mq_provider = RabbitMqProvider::build(publish_channel)
        .await
        .expect("Can't declare a message queue");

    let app = Reconductor::build(db, kv, mq_provider, config);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
