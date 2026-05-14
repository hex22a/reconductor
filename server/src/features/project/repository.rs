use sqlx::PgPool;
use sqlx::types::Uuid;

use crate::features::project::model::{ProjectEntity, ProjectInsert};

pub trait ProjectRepository {
    fn create_project(
        &self,
        project_insert: ProjectInsert,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send;
    fn get_project(
        &self,
        project_id: &Uuid,
    ) -> impl Future<Output = Result<ProjectEntity, sqlx::Error>> + Send;
    fn list_projects(
        &self,
        owner_id: &Uuid,
        cursor_id: Option<&Uuid>,
        limit: i64,
    ) -> impl Future<Output = Result<Vec<ProjectEntity>, sqlx::Error>> + Send;
}

pub struct PgProjectRepository {
    db: PgPool,
}

impl PgProjectRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

impl ProjectRepository for PgProjectRepository {
    async fn create_project(&self, project_insert: ProjectInsert) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO recon.projects
                (name, owner_id)
            VALUES
                ($1, $2)
            "#,
            project_insert.name,
            project_insert.owner_id,
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    async fn get_project(&self, project_id: &Uuid) -> Result<ProjectEntity, sqlx::Error> {
        let project = sqlx::query_as!(
            ProjectEntity,
            r#"
            SELECT
                id,
                owner_id,
                name,
                created_at
            FROM recon.projects
            WHERE id=$1
            LIMIT 1;
            "#,
            project_id
        )
        .fetch_one(&self.db)
        .await?;
        Ok(project)
    }

    async fn list_projects(
        &self,
        owner_id: &Uuid,
        cursor_id: Option<&Uuid>,
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
                    owner_id,
                    cursor,
                    limit
                )
                .fetch_all(&self.db)
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
                    owner_id,
                    limit
                )
                .fetch_all(&self.db)
                .await
            }
        }
    }
}
