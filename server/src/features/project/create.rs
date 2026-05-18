use std::sync::Arc;

use uuid::Uuid;

use crate::features::project::{
    error::ProjectError, model::ProjectInsert, repository::ProjectRepository,
};

pub(crate) trait CreateProjectFeture {
    fn create(
        &self,
        owner_id: Uuid,
        name: String,
    ) -> impl Future<Output = Result<(), ProjectError>> + Send;
}

pub(crate) struct CreateProject<R: ProjectRepository> {
    project_repository: Arc<R>,
}

impl<R: ProjectRepository> CreateProject<R> {
    pub(crate) fn new(project_repository: Arc<R>) -> Self {
        Self { project_repository }
    }
}

impl<R> CreateProjectFeture for CreateProject<R>
where
    R: ProjectRepository + Send + Sync,
{
    async fn create(&self, owner_id: Uuid, name: String) -> Result<(), ProjectError> {
        let project_insert = ProjectInsert { owner_id, name };
        self.project_repository
            .create_project(project_insert)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use uuid::Uuid;

    use crate::features::project::model::ProjectEntity;

    use super::*;
    struct MockProjectRepository {
        error: Mutex<Option<sqlx::Error>>,
    }

    impl ProjectRepository for MockProjectRepository {
        async fn create_project(&self, _: ProjectInsert) -> Result<(), sqlx::Error> {
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(()),
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
        let expected_owner_id = Uuid::now_v7();
        let expected_project_name = "test".to_string();
        let mock_project_repository = MockProjectRepository {
            error: Mutex::new(None),
        };
        let feature = CreateProject::new(Arc::new(mock_project_repository));
        // Act
        let actual_create_project_result = feature
            .create(expected_owner_id, expected_project_name)
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_create_project_result, ());
    }
}
