use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    application::error::{FieldErrors, ServerError},
    features::project::model::CreateProjectInput,
};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProjectDto {
    pub id: Uuid,
    pub name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Deserialize)]
pub struct CreateProjctRequest {
    pub name: String,
}

impl TryFrom<CreateProjctRequest> for CreateProjectInput {
    type Error = ServerError;

    fn try_from(value: CreateProjctRequest) -> Result<Self, Self::Error> {
        if value.name.is_empty() {
            Err(ServerError::ValidationError(FieldErrors::from([(
                "name".to_string(),
                vec!["project name cannot be empty".to_string()],
            )])))
        } else {
            Ok(CreateProjectInput { name: value.name })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::features::project::{dto::CreateProjctRequest, model::CreateProjectInput};

    #[test]
    fn test_valid_project_name() {
        // Arrange
        let expected_project_name = "test".to_string();
        let expected_create_project_input = CreateProjectInput {
            name: expected_project_name.clone(),
        };
        let expected_crate_project_request = CreateProjctRequest {
            name: expected_project_name,
        };
        // Act
        let actual_create_project_input: CreateProjectInput =
            CreateProjectInput::try_from(expected_crate_project_request).unwrap();
        // Assert
        assert_eq!(actual_create_project_input, expected_create_project_input);
    }
}
