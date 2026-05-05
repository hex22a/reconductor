pub struct Config {
    pub database_url: String,
    pub rabbitmq_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            rabbitmq_url: std::env::var("RABBITMQ_URL")?,
        })
    }
}
