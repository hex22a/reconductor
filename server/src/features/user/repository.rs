use std::sync::Arc;

#[cfg(test)]
use mockall::automock;
use sqlx::PgPool;

use crate::{
    domain::username::Username,
    features::user::model::{UserEntity, UserInsert},
};

#[cfg_attr(test, automock)]
pub trait UserRepository {
    fn add_user(
        &self,
        user_insert: UserInsert,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send;
    fn get_user_by_username(
        &self,
        username: &Username,
    ) -> impl Future<Output = Result<UserEntity, sqlx::Error>> + Send;
}

#[derive(Clone)]
pub struct PgUserRepository {
    db: Arc<PgPool>,
}

impl PgUserRepository {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
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
            user_insert.username.as_ref(),
            user_insert.password_hash.as_ref(),
        )
        .execute(&*self.db)
        .await?;
        Ok(())
    }

    async fn get_user_by_username(&self, username: &Username) -> Result<UserEntity, sqlx::Error> {
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
            username.as_ref()
        )
        .fetch_one(&*self.db)
        .await?;
        Ok(user)
    }
}
