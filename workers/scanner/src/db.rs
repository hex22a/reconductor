use sqlx::PgPool;

pub mod scan;

pub async fn init_db(database_url: &str) -> PgPool {
    PgPool::connect(database_url)
        .await
        .expect("failed to connect to db")
}
