use std::{pin::Pin, sync::Arc};

use uuid::Uuid;

use crate::features::project::{
    dto::ProjectDto, error::ProjectError, model::ProjectInsert, repository::ProjectRepository,
};

pub trait CreateProjectFeature {
    fn create(
        &self,
        owner_id: Uuid,
        name: String,
    ) -> Pin<Box<dyn Future<Output = Result<ProjectDto, ProjectError>> + Send + '_>>;
}

pub struct CreateProject<R: ProjectRepository> {
    project_repository: Arc<R>,
}

impl<R: ProjectRepository> CreateProject<R> {
    pub fn new(project_repository: Arc<R>) -> Self {
        Self { project_repository }
    }
}

impl<R> CreateProjectFeature for CreateProject<R>
where
    R: ProjectRepository + Send + Sync,
{
    fn create(
        &self,
        owner_id: Uuid,
        name: String,
    ) -> Pin<Box<dyn Future<Output = Result<ProjectDto, ProjectError>> + Send + '_>> {
        Box::pin(async move {
            let project_insert = ProjectInsert { owner_id, name };
            let project = self
                .project_repository
                .create_project(project_insert)
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
    use uuid::Uuid;

    use crate::features::project::model::ProjectEntity;

    use super::*;
    struct MockProjectRepository {
        error: Mutex<Option<sqlx::Error>>,
        return_value: ProjectEntity,
    }

    impl ProjectRepository for MockProjectRepository {
        async fn create_project(&self, _: ProjectInsert) -> Result<ProjectEntity, sqlx::Error> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(self.return_value.clone()),
            }
        }

        async fn get_project(
            &self,
            _: &uuid::Uuid,
            _: &uuid::Uuid,
        ) -> Result<ProjectEntity, sqlx::Error> {
            todo!()
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
    async fn test_create_project() {
        // Arrange
        let expected_project_id = Uuid::now_v7();
        let expected_owner_id = Uuid::now_v7();
        let expected_project_name = "test".to_string();
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_project_entity = ProjectEntity {
            id: expected_project_id,
            owner_id: expected_owner_id,
            name: expected_project_name.clone(),
            created_at: expected_created_at,
        };
        let expected_project = ProjectDto {
            id: expected_project_id,
            name: expected_project_name.clone(),
            created_at: expected_created_at,
        };
        let mock_project_repository = MockProjectRepository {
            error: Mutex::new(None),
            return_value: expected_project_entity,
        };
        let feature = CreateProject::new(Arc::new(mock_project_repository));
        // Act
        let actual_project = feature
            .create(expected_owner_id, expected_project_name)
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_project, expected_project);
    }
}
