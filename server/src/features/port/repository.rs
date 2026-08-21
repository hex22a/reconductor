use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use crate::features::port::model::PortEntity;

pub trait PortRepository {
    fn get_port(
        &self,
        port_id: &Uuid,
    ) -> impl Future<Output = Result<PortEntity, sqlx::Error>> + Send;

    fn list_ports(
        &self,
        host_id: &Uuid,
        cursor_id: Option<&Uuid>,
        limit: i64,
    ) -> impl Future<Output = Result<Vec<PortEntity>, sqlx::Error>> + Send;
}

pub struct PgPortRepository {
    db: Arc<PgPool>,
}

impl PgPortRepository {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

impl PortRepository for PgPortRepository {
    async fn get_port(&self, port_id: &Uuid) -> Result<PortEntity, sqlx::Error> {
        let port = sqlx::query_as!(
            PortEntity,
            r#"
            SELECT
                id,
                host_id,
                port,
                protocol,
                state,
                service,
                product,
                version
            FROM recon.scan_ports
            WHERE id=$1
            LIMIT 1;
            "#,
            port_id,
        )
        .fetch_one(&*self.db)
        .await?;
        Ok(port)
    }

    async fn list_ports(
        &self,
        host_id: &Uuid,
        cursor_id: Option<&Uuid>,
        limit: i64,
    ) -> Result<Vec<PortEntity>, sqlx::Error> {
        match cursor_id {
            Some(cursor) => {
                sqlx::query_as!(
                    PortEntity,
                    r#"
                    SELECT
                        id,
                        host_id,
                        port,
                        protocol,
                        state,
                        service,
                        product,
                        version
                    FROM recon.scan_ports
                    WHERE host_id=$1 AND id < $2
                    ORDER BY id DESC
                    LIMIT $3;
                    "#,
                    host_id,
                    cursor,
                    limit,
                )
                .fetch_all(&*self.db)
                .await
            }
            None => {
                sqlx::query_as!(
                    PortEntity,
                    r#"
                    SELECT
                        id,
                        host_id,
                        port,
                        protocol,
                        state,
                        service,
                        product,
                        version
                    FROM recon.scan_ports
                    WHERE host_id=$1
                    ORDER BY id DESC
                    LIMIT $2;
                    "#,
                    host_id,
                    limit,
                )
                .fetch_all(&*self.db)
                .await
            }
        }
    }
}
