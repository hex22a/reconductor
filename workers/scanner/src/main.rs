mod nmap;
mod queue;
mod db;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = config::Config::from_env()?;

    let db = db::init_db(&config.database_url).await;
    println!("Hello, world!");
    Ok(())
}
