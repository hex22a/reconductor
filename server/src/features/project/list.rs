use std::{pin::Pin, sync::Arc};

#[cfg(test)]
use mockall::automock;

use uuid::Uuid;

use crate::{
    constants::PROJECTS_PAGE_SIZE_LIMIT,
    domain::cursor::{decode_cursor, encode_cursor},
    features::project::{dto::ProjectDto, error::ProjectError, repository::ProjectRepository},
    transport::pagination::{Page, PageInfo},
};

#[cfg_attr(test, automock)]
pub trait ListProjectsFeature {
    fn list<'a>(
        &'a self,
        owner_id: Uuid,
        cursor_id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Page<ProjectDto>, ProjectError>> + Send + 'a>>;
}

#[derive(Clone)]
pub struct ListProjects<P: ProjectRepository> {
    project_repository: Arc<P>,
}

impl<P: ProjectRepository> ListProjects<P> {
    pub fn new(project_repository: Arc<P>) -> Self {
        Self { project_repository }
    }
}

impl<P> ListProjectsFeature for ListProjects<P>
where
    P: ProjectRepository + Send + Sync,
{
    fn list<'a>(
        &'a self,
        owner_id: Uuid,
        cursor_id: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Page<ProjectDto>, ProjectError>> + Send + 'a>> {
        Box::pin(async move {
            let mut has_next_page = false;
            let maybe_cursor_id = cursor_id.map(|cursor| decode_cursor(&cursor)).transpose()?;
            let limit = PROJECTS_PAGE_SIZE_LIMIT + 1;
            let mut projects = self
                .project_repository
                .list_projects(&owner_id, maybe_cursor_id, limit)
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
        })
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    use crate::{
        constants::PROJECTS_PAGE_SIZE_LIMIT,
        domain::cursor::encode_cursor,
        features::project::{
            dto::ProjectDto, model::ProjectEntity, repository::MockProjectRepository,
        },
        transport::pagination::PageInfo,
    };

    #[tokio::test]
    async fn test_list_projects_no_next_page() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_project_id = Uuid::now_v7();
        let expected_owner_id = Uuid::now_v7();
        let expected_name = "test".to_string();
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_project_entities_size = PROJECTS_PAGE_SIZE_LIMIT as usize;
        let expected_project = ProjectEntity {
            id: expected_project_id,
            owner_id: expected_owner_id,
            name: expected_name.clone(),
            created_at: expected_created_at,
        };
        let expected_project_entities = vec![expected_project; expected_project_entities_size];
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

        let mut mock_project_repository = MockProjectRepository::new();
        mock_project_repository
            .expect_list_projects()
            .returning(move |_, _, _| {
                let project_entities = expected_project_entities.clone();
                Box::pin(async { Ok(project_entities) })
            });

        let feature = ListProjects::new(Arc::new(mock_project_repository));

        // Act
        let actual_page = feature
            .list(expected_owner_id, Some(expected_cursor_id))
            .await
            .unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_projects_with_next_page() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_project_id = Uuid::now_v7();
        let expected_end_cursor = encode_cursor(&expected_project_id);
        let expected_owner_id = Uuid::now_v7();
        let expected_name = "test".to_string();
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_project_entities_size = PROJECTS_PAGE_SIZE_LIMIT as usize;
        let expected_project = ProjectEntity {
            id: expected_project_id,
            owner_id: expected_owner_id,
            name: expected_name.clone(),
            created_at: expected_created_at,
        };
        let expected_project_entities = vec![expected_project; expected_project_entities_size + 1];
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

        let mut mock_project_repository = MockProjectRepository::new();
        mock_project_repository
            .expect_list_projects()
            .returning(move |_, _, _| {
                let project_entities = expected_project_entities.clone();
                Box::pin(async { Ok(project_entities) })
            });

        let feature = ListProjects::new(Arc::new(mock_project_repository));

        // Act
        let actual_page = feature
            .list(expected_owner_id, Some(expected_cursor_id))
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
        let expected_created_at = datetime!(2019-01-01 0:00 UTC);
        let expected_project_entities_size = PROJECTS_PAGE_SIZE_LIMIT as usize;
        let expected_project = ProjectEntity {
            id: expected_project_id,
            owner_id: expected_owner_id,
            name: expected_name.clone(),
            created_at: expected_created_at,
        };
        let expected_project_entities = vec![expected_project; expected_project_entities_size + 1];
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

        let mut mock_project_repository = MockProjectRepository::new();
        mock_project_repository
            .expect_list_projects()
            .returning(move |_, _, _| {
                let project_entities = expected_project_entities.clone();
                Box::pin(async { Ok(project_entities) })
            });

        let feature = ListProjects::new(Arc::new(mock_project_repository));

        // Act
        let actual_page = feature.list(expected_owner_id, None).await.unwrap();

        // Assert
        assert_eq!(actual_page, expected_page);
    }

    #[tokio::test]
    async fn test_list_projects_not_found() {
        // Arrange
        let expected_cursor_id = String::from("AZ0GNLkMdACZ0iU9dt-z6g");
        let expected_owner_id = Uuid::now_v7();

        let mut mock_project_repository = MockProjectRepository::new();
        mock_project_repository
            .expect_list_projects()
            .returning(|_, _, _| Box::pin(async { Err(sqlx::Error::RowNotFound) }));

        let feature = ListProjects::new(Arc::new(mock_project_repository));
        // Act
        let actual_result = feature
            .list(expected_owner_id, Some(expected_cursor_id))
            .await;
        // Assert
        assert!(matches!(actual_result, Err(ProjectError::NotFound)));
    }
}
