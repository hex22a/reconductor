mod nmap;
mod queue;
mod db;
mod config;

use crate::db::scan::PgScanRepository;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = config::Config::from_env()?;
    let conn = lapin::Connection::connect(
        &config.rabbitmq_url,
        lapin::ConnectionProperties::default(),
    )
    .await?;
    info!("Connected to RabbitMQ");

    let consume_channel = conn.create_channel().await?;
    let db = db::init_db(&config.database_url).await;
    let repository = PgScanRepository { db };

    queue::consumer::run(repository, consume_channel).await?;

    Ok(())
}
