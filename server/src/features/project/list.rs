use std::sync::Arc;

use uuid::Uuid;

use crate::{
    constants::PROJECTS_PAGE_SIZE_LIMIT,
    domain::cursor::{decode_cursor, encode_cursor},
    features::project::{dto::ProjectDto, error::ProjectError, repository::ProjectRepository},
    transport::pagination::{Page, PageInfo},
};

pub trait ListProjectsFeature {
    fn list(
        &self,
        owner_id: &Uuid,
        cursor_id: Option<&str>,
    ) -> impl Future<Output = Result<Page<ProjectDto>, ProjectError>> + Send;
}

#[derive(Clone)]
pub struct ListProjects<P: ProjectRepository> {
    project_repository: Arc<P>,
}

impl<P: ProjectRepository> ListProjects<P> {
    fn new(project_repository: Arc<P>) -> Self {
        Self { project_repository }
    }
}

impl<P> ListProjectsFeature for ListProjects<P>
where
    P: ProjectRepository + Send + Sync,
{
    async fn list(
        &self,
        owner_id: &Uuid,
        cursor_id: Option<&str>,
    ) -> Result<Page<ProjectDto>, ProjectError> {
        let mut has_next_page = false;
        let maybe_cursor_id = cursor_id.map(decode_cursor).transpose()?;
        let maybe_cursor_id_ref = maybe_cursor_id.as_ref();
        let limit = PROJECTS_PAGE_SIZE_LIMIT + 1;
        let mut projects = self
            .project_repository
            .list_projects(owner_id, maybe_cursor_id_ref, limit)
            .await?;
        if projects.len() == limit as usize {
            has_next_page = true;
            projects.pop();
        }
        let project_dtos = projects
            .iter()
            .map(|p| ProjectDto {
                id: p.id,
                name: p.name.clone(),
                created_at: p.created_at,
            })
            .collect();
        Ok(Page {
            data: project_dtos,
            page_info: PageInfo {
                has_next_page,
                end_cursor: match has_next_page {
                    true => Some(encode_cursor(
                        &projects.last().ok_or(ProjectError::NoLastCursor)?.id,
                    )),
                    false => None,
                },
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;
    use std::sync::Mutex;

    use crate::{
        constants::PROJECTS_PAGE_SIZE_LIMIT,
        domain::cursor::encode_cursor,
        features::project::{
            dto::ProjectDto,
            model::{ProjectEntity, ProjectInsert},
            repository::ProjectRepository,
        },
        transport::pagination::PageInfo,
    };

    struct MockProjectRepository {
        error: Mutex<Option<sqlx::Error>>,
        project_entity: ProjectEntity,
        size: usize,
    }

    impl ProjectRepository for MockProjectRepository {
        async fn create_project(&self, _: ProjectInsert) -> Result<(), sqlx::Error> {
            todo!()
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
            match self.error.lock().unwrap().take() {
                Some(e) => Err(e),
                None => Ok(vec![self.project_entity.clone(); self.size]),
            }
        }
    }

    #[tokio::test]
    async fn test_list_projects_no_next_page() {
        // Arrange
        let expected_cursor_id = "AZ0GNLkMdACZ0iU9dt-z6g";
        let expected_project_id = Uuid::now_v7();
        let expected_owner_id = Uuid::now_v7();
        let expected_name = "test".to_string();
        let expected_created_at = datetime!(2019-01-01 0:00);
        let expected_project_entities_size = PROJECTS_PAGE_SIZE_LIMIT as usize;
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
        let expected_projects = vec![expected_project_dto; expected_project_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: false,
            end_cursor: None,
        };
        let expected_page = Page::<ProjectDto> {
            data: expected_projects,
            page_info: expected_page_info,
        };
        let mock_project_repository = MockProjectRepository {
            error: Mutex::new(None),
            project_entity: expected_project,
            size: expected_project_entities_size,
        };
        let feature = ListProjects::new(Arc::new(mock_project_repository));
        // Act
        let actual_page = feature
            .list(&expected_owner_id, Some(&expected_cursor_id))
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_projects_with_next_page() {
        // Arrange
        let expected_cursor_id = "AZ0GNLkMdACZ0iU9dt-z6g";
        let expected_project_id = Uuid::now_v7();
        let expected_end_cursor = encode_cursor(&expected_project_id);
        let expected_owner_id = Uuid::now_v7();
        let expected_name = "test".to_string();
        let expected_created_at = datetime!(2019-01-01 0:00);
        let expected_project_entities_size = PROJECTS_PAGE_SIZE_LIMIT as usize;
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
        let expected_projects = vec![expected_project_dto; expected_project_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: true,
            end_cursor: Some(expected_end_cursor),
        };
        let expected_page = Page::<ProjectDto> {
            data: expected_projects,
            page_info: expected_page_info,
        };
        let mock_project_repository = MockProjectRepository {
            error: Mutex::new(None),
            project_entity: expected_project,
            size: expected_project_entities_size + 1,
        };
        let feature = ListProjects::new(Arc::new(mock_project_repository));
        // Act
        let actual_page = feature
            .list(&expected_owner_id, Some(&expected_cursor_id))
            .await
            .unwrap();
        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_projects_no_cursor() {
        // Arrange
        let expected_project_id = Uuid::now_v7();
        let expected_end_cursor = encode_cursor(&expected_project_id);
        let expected_owner_id = Uuid::now_v7();
        let expected_name = "test".to_string();
        let expected_created_at = datetime!(2019-01-01 0:00);
        let expected_project_entities_size = PROJECTS_PAGE_SIZE_LIMIT as usize;
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
        let expected_projects = vec![expected_project_dto; expected_project_entities_size];
        let expected_page_info = PageInfo {
            has_next_page: true,
            end_cursor: Some(expected_end_cursor),
        };
        let expected_page = Page::<ProjectDto> {
            data: expected_projects,
            page_info: expected_page_info,
        };
        let mock_project_repository = MockProjectRepository {
            error: Mutex::new(None),
            project_entity: expected_project,
            size: expected_project_entities_size + 1,
        };
        let feature = ListProjects::new(Arc::new(mock_project_repository));
        // Act
        let actual_page = feature.list(&expected_owner_id, None).await.unwrap();
        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_projects_not_found() {
        // Arrange
        let expected_cursor_id = "AZ0GNLkMdACZ0iU9dt-z6g";
        let expected_project_id = Uuid::now_v7();
        let expected_owner_id = Uuid::now_v7();
        let expected_name = "test".to_string();
        let expected_created_at = datetime!(2019-01-01 0:00);
        let expected_project_entities_size = PROJECTS_PAGE_SIZE_LIMIT as usize;
        let expected_project = ProjectEntity {
            id: expected_project_id,
            owner_id: expected_owner_id,
            name: expected_name.clone(),
            created_at: expected_created_at,
        };
        let mock_project_repository = MockProjectRepository {
            error: Mutex::new(Some(sqlx::Error::RowNotFound)),
            project_entity: expected_project,
            size: expected_project_entities_size,
        };
        let feature = ListProjects::new(Arc::new(mock_project_repository));
        // Act
        let actual_result = feature
            .list(&expected_owner_id, Some(&expected_cursor_id))
            .await;
        // Assert
        assert!(matches!(actual_result, Err(ProjectError::NotFound)));
    }
}
