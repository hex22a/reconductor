use scheduler::application::error::AppError;

pub struct Config {
    pub database_url: String,
    pub rabbitmq_host: String,
    pub rabbitmq_port: u16,
    pub rabbitmq_vhost: String,
    pub rabbitmq_username: String,
    pub rabbitmq_password: String,
    pub poll_interval_secs: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            rabbitmq_host: std::env::var("RABBITMQ_HOST")?,
            rabbitmq_port: std::env::var("RABBITMQ_PORT")?.parse()?,
            rabbitmq_vhost: std::env::var("RABBITMQ_VHOST")?,
            rabbitmq_username: std::env::var("RABBITMQ_USERNAME")?,
            rabbitmq_password: std::env::var("RABBITMQ_PASSWORD")?,
            poll_interval_secs: std::env::var("POLL_INTERVAL_SECS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()?,
        })
    }
}
