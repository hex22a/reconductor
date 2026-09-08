use scheduler::application::error::AppError;

pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub db_username: String,
    pub db_password: String,
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
            db_host: std::env::var("DB_HOST")?,
            db_port: std::env::var("DB_PORT")?.parse()?,
            db_name: std::env::var("DB_NAME")?,
            db_username: std::env::var("DB_USERNAME")?,
            db_password: std::env::var("DB_PASSWORD")?,
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
