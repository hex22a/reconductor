use axum::http::HeaderValue;

use crate::AppError;

pub struct Config {
    pub database_url: String,
    pub kv_host: String,
    pub kv_port: u16,
    pub kv_username: String,
    pub kv_password: String,
    pub kv_db: u8,
    pub rabbitmq_url: String,
    pub csrf_key: [u8; 32],
    pub dashboard_url: HeaderValue,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            kv_host: std::env::var("KV_HOST")?,
            kv_port: std::env::var("KV_PORT")?.parse()?,
            kv_username: std::env::var("KV_USERNAME")?,
            kv_password: std::env::var("KV_PASSWORD")?,
            kv_db: std::env::var("KV_DB")?.parse()?,
            rabbitmq_url: std::env::var("RABBITMQ_URL")?,
            csrf_key: hex::decode(std::env::var("CSRF_SECRET")?)?
                .try_into()
                .map_err(|_| AppError::CsrfLengthError)?,
            dashboard_url: std::env::var("DASHBOARD_URL")?.parse::<HeaderValue>()?,
        })
    }
}
