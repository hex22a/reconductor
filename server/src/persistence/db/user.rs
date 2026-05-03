use sqlx::{
    PgPool,
    types::{Uuid, time::PrimitiveDateTime},
};

pub struct UserEntity {
    id: Uuid,
    pub username: String,
    pub password_hash: String,
    password_version: i16,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,
    last_login_at: PrimitiveDateTime,
    is_active: bool,
}

pub struct UserInsert {
    pub username: String,
    pub password_hash: String,
}

#[allow(async_fn_in_trait)]
pub trait UserRepository {
    async fn add_user(&self, user_insert: UserInsert) -> Result<(), sqlx::Error>;
    async fn get_user_by_username(&self, username: &str) -> Result<UserEntity, sqlx::Error>;
}

pub struct PgUserRepository {
    pub db: PgPool,
}

impl UserRepository for PgUserRepository {
    async fn add_user(&self, user_insert: UserInsert) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO recon.users
                (username, password_hash)
            VALUES
                ($1, $2)
            "#,
            user_insert.username,
            user_insert.password_hash,
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    async fn get_user_by_username(&self, username: &str) -> Result<UserEntity, sqlx::Error> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT
                id,
                username,
                password_hash,
                password_version,
                created_at,
                updated_at,
                last_login_at,
                is_active
            FROM recon.users
            WHERE username=$1
            LIMIT 1;
            "#,
            username
        )
        .fetch_one(&self.db)
        .await?;
        Ok(user)
    }
}
