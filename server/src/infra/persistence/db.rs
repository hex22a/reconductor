use sqlx::PgPool;

pub(crate) async fn init_db(database_url: &str) -> PgPool {
    PgPool::connect(database_url)
        .await
        .expect("failed to connect to db")
}
