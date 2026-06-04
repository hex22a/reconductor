use std::collections::HashMap;

use regex::Regex;
use serde::Deserialize;
use sqlx::types::ipnetwork::IpNetwork;

use crate::{application::error::ServerError, features::scan::model::CreateScanInput};

#[derive(Deserialize)]
pub(crate) struct CreateScanRequest {
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
        let schedule = value.schedule.inspect(|s| {
            let cron_regex = Regex::new(r"^(\*|([0-5]?\d)) (\*|([01]?\d|2[0-3])) (\*|([12]?\d|3[01])) (\*|(1[0-2]|[1-9])) (\*|([0-7]))$").unwrap();
            if !cron_regex.is_match(s) {
                error = true;
                field_errors.insert("schedule".to_string(), vec!["invalid cron expression".to_string()]);
            }
        });
        if error {
            return Err(ServerError::ValidationError(field_errors));
        }
        Ok(CreateScanInput {
            target: target.unwrap(),
            schedule,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::error::ServerError,
        features::scan::{dto::CreateScanRequest, model::CreateScanInput},
    };

    #[test]
    fn test_valid_target_valid_schedule() {
        // Arrange
        let expected_target = "192.168.0.1";
        let expected_schedule = "* * * * *".to_string();
        let expected_create_scan_request = CreateScanRequest {
            target: expected_target.to_string(),
            schedule: Some(expected_schedule.clone()),
        };
        let expected_create_scan_input = CreateScanInput {
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
        let expected_target = "192.168.0.1";
        let expected_create_scan_request = CreateScanRequest {
            target: expected_target.to_string(),
            schedule: None,
        };
        let expected_create_scan_input = CreateScanInput {
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
        let expected_target = "300.168.0.1";
        let expected_create_scan_request = CreateScanRequest {
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
        let expected_target = "192.168.0.1";
        let expected_schedule = "200 * * * *".to_string();
        let expected_create_scan_request = CreateScanRequest {
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
