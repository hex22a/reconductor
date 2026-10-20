use crate::{domain::types::ProjectId, features::scan::model::ScanStatus};
use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

use reconductor_core::ScanId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::features::scan::model::{ScanEntity, ScanInsert};

#[cfg_attr(test, automock)]
pub trait ScanRepository {
    fn create_scan(
        &self,
        scan_insert: ScanInsert,
    ) -> impl Future<Output = Result<ScanEntity, sqlx::Error>> + Send;
    fn get_scan(
        &self,
        scan_id: &ScanId,
    ) -> impl Future<Output = Result<ScanEntity, sqlx::Error>> + Send;
    fn list_scans(
        &self,
        project_id: &ProjectId,
        cursor_id: Option<Uuid>,
        limit: i64,
    ) -> impl Future<Output = Result<Vec<ScanEntity>, sqlx::Error>> + Send;
}

pub struct PgScanRespository {
    db: Arc<PgPool>,
}

impl PgScanRespository {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

impl ScanRepository for PgScanRespository {
    async fn create_scan(&self, scan_insert: ScanInsert) -> Result<ScanEntity, sqlx::Error> {
        let scan = sqlx::query_as!(
            ScanEntity,
            r#"
            INSERT INTO recon.scans
                (project_id, target, schedule, next_run_at)
            VALUES
                ($1, $2, $3, $4)
            RETURNING
                id,
                project_id,
                created_at,
                target,
                status as "status: ScanStatus",
                schedule,
                next_run_at
            "#,
            scan_insert.project_id.as_ref(),
            scan_insert.target.as_ref(),
            scan_insert.schedule,
            scan_insert.next_run_at,
        )
        .fetch_one(&*self.db)
        .await?;
        Ok(scan)
    }

    async fn get_scan(&self, scan_id: &ScanId) -> Result<ScanEntity, sqlx::Error> {
        let scan = sqlx::query_as!(
            ScanEntity,
            r#"
            SELECT
                id,
                project_id,
                created_at,
                target,
                status as "status: ScanStatus",
                schedule,
                next_run_at
            FROM recon.scans
            WHERE id=$1
            LIMIT 1;
            "#,
            scan_id.as_ref(),
        )
        .fetch_one(&*self.db)
        .await?;
        Ok(scan)
    }

    async fn list_scans(
        &self,
        project_id: &ProjectId,
        cursor_id: Option<Uuid>,
        limit: i64,
    ) -> Result<Vec<ScanEntity>, sqlx::Error> {
        match cursor_id {
            Some(cursor) => {
                sqlx::query_as!(
                    ScanEntity,
                    r#"
                    SELECT
                        id,
                        project_id,
                        created_at,
                        target,
                        status as "status: ScanStatus",
                        schedule,
                        next_run_at
                    FROM recon.scans
                    WHERE project_id=$1 and id < $2
                    ORDER BY id DESC
                    LIMIT $3;
                    "#,
                    project_id.as_ref(),
                    cursor,
                    limit,
                )
                .fetch_all(&*self.db)
                .await
            }
            None => {
                sqlx::query_as!(
                    ScanEntity,
                    r#"
                    SELECT
                        id,
                        project_id,
                        created_at,
                        target,
                        status as "status: ScanStatus",
                        schedule,
                        next_run_at
                    FROM recon.scans
                    WHERE project_id=$1
                    ORDER BY id DESC
                    LIMIT $2;
                    "#,
                    project_id.as_ref(),
                    limit,
                )
                .fetch_all(&*self.db)
                .await
            }
        }
    }
}
