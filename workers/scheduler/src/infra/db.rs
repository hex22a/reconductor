use sqlx::{PgPool, postgres::PgConnectOptions};

pub struct DbConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

pub async fn init_db(config: DbConfig) -> PgPool {
    let options = PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.username)
        .password(&config.password)
        .database(&config.db_name);
    PgPool::connect_with(options)
        .await
        .expect("failed to connect to db")
}
