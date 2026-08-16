use std::{collections::HashMap, str::FromStr};

use cron::Schedule;
use serde::{Deserialize, Serialize};
use sqlx::types::ipnetwork::IpNetwork;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{application::error::ServerError, features::scan::model::CreateScanInput};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct ScanDto {
    pub(crate) id: Uuid,
    pub(crate) target: IpNetwork,
    pub(crate) schedule: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) created_at: OffsetDateTime,
}

#[derive(Deserialize)]
pub(crate) struct CreateScanRequest {
    pub project_id: Uuid,
    pub(crate) target: String,
    pub(crate) schedule: Option<String>,
}

impl TryFrom<CreateScanRequest> for CreateScanInput {
    type Error = ServerError;

    fn try_from(value: CreateScanRequest) -> Result<Self, Self::Error> {
        let mut field_errors: HashMap<String, Vec<String>> = HashMap::new();
        let mut error = false;
        let target: Result<IpNetwork, _> = value.target.parse();
        if target.is_err() {
            error = true;
            field_errors.insert("target".to_string(), vec!["invalid ip address".to_string()]);
        }
        let mut schedule: Option<Schedule> = None;
        if let Some(s) = value.schedule {
            let with_seconds = format!("0 {}", s);
            match Schedule::from_str(&with_seconds) {
                Ok(parsed) => schedule = Some(parsed),
                Err(_) => {
                    error = true;
                    field_errors.insert(
                        "schedule".to_string(),
                        vec!["invalid cron expression".to_string()],
                    );
                }
            }
        }
        if error {
            return Err(ServerError::ValidationError(field_errors));
        }
        Ok(CreateScanInput {
            project_id: value.project_id,
            target: target.unwrap(),
            schedule,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cron::Schedule;
    use uuid::Uuid;

    use crate::{
        application::error::ServerError,
        features::scan::{dto::CreateScanRequest, model::CreateScanInput},
    };

    #[test]
    fn test_valid_target_valid_schedule() {
        // Arrange
        let expected_project_id = Uuid::now_v7();
        let expected_target = "192.168.0.1";
        let expected_schedule_string = "* * * * *".to_string();
        let expected_schedule_with_seconds_string = "0 * * * * *".to_string();
        let expected_schedule = Schedule::from_str(&expected_schedule_with_seconds_string).unwrap();
        let expected_create_scan_request = CreateScanRequest {
            project_id: expected_project_id.clone(),
            target: expected_target.to_string(),
            schedule: Some(expected_schedule_string.clone()),
        };
        let expected_create_scan_input = CreateScanInput {
            project_id: expected_project_id,
            target: expected_target.try_into().unwrap(),
            schedule: Some(expected_schedule),
        };
        // Act
        let actual_create_scan_input: CreateScanInput =
            CreateScanInput::try_from(expected_create_scan_request).unwrap();
        // Assert
        assert_eq!(actual_create_scan_input, expected_create_scan_input);
    }

    #[test]
    fn test_valid_target_empty_schedule() {
        // Arrange
        let expected_project_id = Uuid::now_v7();
        let expected_target = "192.168.0.1";
        let expected_create_scan_request = CreateScanRequest {
            project_id: expected_project_id.clone(),
            target: expected_target.to_string(),
            schedule: None,
        };
        let expected_create_scan_input = CreateScanInput {
            project_id: expected_project_id,
            target: expected_target.try_into().unwrap(),
            schedule: None,
        };
        // Act
        let actual_create_scan_input: CreateScanInput =
            CreateScanInput::try_from(expected_create_scan_request).unwrap();
        // Assert
        assert_eq!(actual_create_scan_input, expected_create_scan_input);
    }

    #[test]
    fn test_invalid_target() {
        // Arrange
        let expected_project_id = Uuid::now_v7();
        let expected_target = "300.168.0.1";
        let expected_create_scan_request = CreateScanRequest {
            project_id: expected_project_id,
            target: expected_target.to_string(),
            schedule: None,
        };
        // Act
        let actualt_try_from_result = CreateScanInput::try_from(expected_create_scan_request);
        // Assert
        assert!(matches!(
            actualt_try_from_result,
            Err(ServerError::ValidationError(_))
        ));
    }

    #[test]
    fn test_invalid_schedule() {
        // Arrange
        let expected_project_id = Uuid::now_v7();
        let expected_target = "192.168.0.1";
        let expected_schedule = "200 * * * *".to_string();
        let expected_create_scan_request = CreateScanRequest {
            project_id: expected_project_id,
            target: expected_target.to_string(),
            schedule: Some(expected_schedule),
        };
        // Act
        let actualt_try_from_result = CreateScanInput::try_from(expected_create_scan_request);
        // Assert
        assert!(matches!(
            actualt_try_from_result,
            Err(ServerError::ValidationError(_))
        ));
    }
}
