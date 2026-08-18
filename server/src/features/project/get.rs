use std::{pin::Pin, sync::Arc};

use uuid::Uuid;

use crate::features::project::{
    dto::ProjectDto, error::ProjectError, repository::ProjectRepository,
};

pub trait GetProjectFeature {
    fn get(
        &self,
        project_id: Uuid,
        owner_id: Uuid,
    ) -> Pin<Box<dyn Future<Output = Result<ProjectDto, ProjectError>> + Send + '_>>;
}

pub struct GetProject<R: ProjectRepository> {
    project_repository: Arc<R>,
}

impl<R: ProjectRepository> GetProject<R> {
    pub fn new(project_repository: Arc<R>) -> Self {
        Self { project_repository }
    }
}

impl<R> GetProjectFeature for GetProject<R>
where
    R: ProjectRepository + Send + Sync,
{
    fn get(
        &self,
        project_id: Uuid,
        owner_id: Uuid,
    ) -> Pin<Box<dyn Future<Output = Result<ProjectDto, ProjectError>> + Send + '_>> {
        Box::pin(async move {
            let project = self
                .project_repository
                .get_project(&project_id, &owner_id)
                .await?;
            Ok(ProjectDto {
                id: project.id,
                name: project.name,
                created_at: project.created_at,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use time::macros::datetime;

    use crate::features::project::model::{ProjectEntity, ProjectInsert};

    use super::*;

    struct MockProjectRepository {
        error: Mutex<Option<sqlx::Error>>,
        project_entity: ProjectEntity,
    }

    impl ProjectRepository for MockProjectRepository {
        async fn create_project(&self, _: ProjectInsert) -> Result<ProjectEntity, sqlx::Error> {
            todo!()
        }

        async fn get_project(
            &self,
            _: &uuid::Uuid,
            _: &uuid::Uuid,
        ) -> Result<ProjectEntity, sqlx::Error> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(self.project_entity.clone()),
            }
        }

        async fn list_projects(
            &self,
            _: &uuid::Uuid,
            _: Option<&uuid::Uuid>,
            _: i64,
        ) -> Result<Vec<ProjectEntity>, sqlx::Error> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn test_get_project() {
        // Arrange
        let expected_project_id = Uuid::now_v7();
        let expected_owner_id = Uuid::now_v7();
        let expected_name = "test".to_string();
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_project = ProjectEntity {
            id: expected_project_id,
            owner_id: expected_owner_id,
            name: expected_name.clone(),
            created_at: expected_created_at,
        };
        let expected_project_dto = ProjectDto {
            id: expected_project_id,
            name: expected_name,
            created_at: expected_created_at,
        };
        let mock_project_repository = MockProjectRepository {
            error: Mutex::new(None),
            project_entity: expected_project,
        };
        let feature = GetProject::new(Arc::new(mock_project_repository));
        // Act
        let actual_project_dto = feature
            .get(expected_project_id, expected_owner_id)
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_project_dto, expected_project_dto);
    }

    #[tokio::test]
    async fn test_get_project_not_found() {
        // Arrange
        let expected_project_id = Uuid::now_v7();
        let expected_owner_id = Uuid::now_v7();
        let expected_name = "test".to_string();
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_project = ProjectEntity {
            id: expected_project_id,
            owner_id: expected_owner_id,
            name: expected_name.clone(),
            created_at: expected_created_at,
        };
        let mock_project_repository = MockProjectRepository {
            error: Mutex::new(Some(sqlx::Error::RowNotFound)),
            project_entity: expected_project,
        };
        let feature = GetProject::new(Arc::new(mock_project_repository));
        // Act
        let actual_result = feature.get(expected_project_id, expected_owner_id).await;
        // Assert
        assert!(matches!(actual_result, Err(ProjectError::NotFound)));
    }
}
