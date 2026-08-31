use axum::http::HeaderValue;

use crate::AppError;

pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub rabbitmq_url: String,
    pub csrf_key: [u8; 32],
    pub dashboard_url: HeaderValue,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            redis_url: std::env::var("REDIS_URL")?,
            rabbitmq_url: std::env::var("RABBITMQ_URL")?,
            csrf_key: hex::decode(std::env::var("CSRF_SECRET")?)?
                .try_into()
                .map_err(|_| AppError::CsrfLengthError)?,
            dashboard_url: std::env::var("DASHBOARD_URL")?.parse::<HeaderValue>()?,
        })
    }
}
