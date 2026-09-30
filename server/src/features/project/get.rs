use std::{pin::Pin, sync::Arc};

#[cfg(test)]
use mockall::automock;

use uuid::Uuid;

use crate::features::project::{
    dto::ProjectDto, error::ProjectError, repository::ProjectRepository,
};

#[cfg_attr(test, automock)]
pub trait GetProjectFeature {
    fn get<'a>(
        &'a self,
        project_id: Uuid,
        owner_id: Uuid,
    ) -> Pin<Box<dyn Future<Output = Result<ProjectDto, ProjectError>> + Send + 'a>>;
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
    use time::macros::datetime;

    use crate::features::project::{model::ProjectEntity, repository::MockProjectRepository};

    use super::*;

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

        let mut mock_project_repository = MockProjectRepository::new();
        mock_project_repository
            .expect_get_project()
            .returning(move |_, _| {
                let project = expected_project.clone();
                Box::pin(async { Ok(project) })
            });

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

        let mut mock_project_repository = MockProjectRepository::new();
        mock_project_repository
            .expect_get_project()
            .returning(|_, _| Box::pin(async { Err(sqlx::Error::RowNotFound) }));

        let feature = GetProject::new(Arc::new(mock_project_repository));

        // Act
        let actual_result = feature.get(expected_project_id, expected_owner_id).await;

        // Assert
        assert!(matches!(actual_result, Err(ProjectError::NotFound)));
    }
}
