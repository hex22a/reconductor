use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

use sqlx::PgPool;
use sqlx::types::Uuid;

use crate::{
    domain::types::{ProjectId, UserId},
    features::project::model::{ProjectEntity, ProjectInsert},
};

#[cfg_attr(test, automock)]
pub trait ProjectRepository {
    fn create_project(
        &self,
        project_insert: ProjectInsert,
    ) -> impl Future<Output = Result<ProjectEntity, sqlx::Error>> + Send;
    fn get_project(
        &self,
        project_id: &ProjectId,
        owner_id: &UserId,
    ) -> impl Future<Output = Result<ProjectEntity, sqlx::Error>> + Send;
    fn list_projects(
        &self,
        owner_id: &UserId,
        cursor_id: Option<Uuid>,
        limit: i64,
    ) -> impl Future<Output = Result<Vec<ProjectEntity>, sqlx::Error>> + Send;
}

pub struct PgProjectRepository {
    db: Arc<PgPool>,
}

impl PgProjectRepository {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

impl ProjectRepository for PgProjectRepository {
    async fn create_project(
        &self,
        project_insert: ProjectInsert,
    ) -> Result<ProjectEntity, sqlx::Error> {
        let project = sqlx::query_as!(
            ProjectEntity,
            r#"
            INSERT INTO recon.projects
                (name, owner_id)
            VALUES
                ($1, $2)
            RETURNING *;
            "#,
            project_insert.name,
            project_insert.owner_id.into_inner(),
        )
        .fetch_one(&*self.db)
        .await?;
        Ok(project)
    }

    async fn get_project(
        &self,
        project_id: &ProjectId,
        owner_id: &UserId,
    ) -> Result<ProjectEntity, sqlx::Error> {
        let project = sqlx::query_as!(
            ProjectEntity,
            r#"
            SELECT
                id,
                owner_id,
                name,
                created_at
            FROM recon.projects
            WHERE id=$1 and owner_id=$2
            LIMIT 1;
            "#,
            project_id.as_ref(),
            owner_id.as_ref(),
        )
        .fetch_one(&*self.db)
        .await?;
        Ok(project)
    }

    async fn list_projects(
        &self,
        owner_id: &UserId,
        cursor_id: Option<Uuid>,
        limit: i64,
    ) -> Result<Vec<ProjectEntity>, sqlx::Error> {
        match cursor_id {
            Some(cursor) => {
                sqlx::query_as!(
                    ProjectEntity,
                    r#"
                    SELECT
                        id,
                        owner_id,
                        name,
                        created_at
                    FROM recon.projects
                    WHERE owner_id = $1 AND id < $2
                    ORDER BY id DESC
                    LIMIT $3;
                    "#,
                    owner_id.as_ref(),
                    cursor,
                    limit
                )
                .fetch_all(&*self.db)
                .await
            }
            None => {
                sqlx::query_as!(
                    ProjectEntity,
                    r#"
                    SELECT
                        id,
                        owner_id,
                        name,
                        created_at
                    FROM recon.projects
                    WHERE owner_id = $1
                    ORDER BY id DESC
                    LIMIT $2;
                    "#,
                    owner_id.as_ref(),
                    limit
                )
                .fetch_all(&*self.db)
                .await
            }
        }
    }
}
