use scanner::AppError;

pub struct Config {
    pub database_url: String,
    pub rabbitmq_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            rabbitmq_url: std::env::var("RABBITMQ_URL")?,
        })
    }
}
