pub struct Config {
    pub database_url: String,
    pub rabbitmq_url: String,
    pub poll_interval_secs: u64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            rabbitmq_url: std::env::var("RABBITMQ_URL")?,
            poll_interval_secs: std::env::var("POLL_INTERVAL_SECS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()?,
        })
    }
}
