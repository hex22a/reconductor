use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    application::error::ServerError,
    constants::{
        MAX_PASSWORD_LENGTH, MAX_USERNAME_LENGTH, MIN_PASSWORD_LENGTH, MIN_USERNAME_LENGTH,
    },
    features::user::model::UserInput,
};

#[derive(Deserialize, Serialize)]
pub struct UserInputRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub csrf_token: String,
}

#[derive(Serialize)]
pub struct MeResponse {
    pub username: String,
}

impl TryFrom<UserInputRequest> for UserInput {
    type Error = ServerError;

    fn try_from(value: UserInputRequest) -> Result<Self, Self::Error> {
        let username_len = value.username.len();
        let password_len = value.password.len();
        let mut field_errors: HashMap<String, Vec<String>> = HashMap::new();
        let mut error = false;
        if username_len < MIN_USERNAME_LENGTH || username_len > MAX_USERNAME_LENGTH {
            error = true;
            field_errors.insert(
                "username".to_string(),
                vec![format!(
                    "username must be at least {min} characters and no loner than {max} characters",
                    min = MIN_USERNAME_LENGTH,
                    max = MAX_USERNAME_LENGTH
                )],
            );
        };
        if password_len < MIN_PASSWORD_LENGTH || password_len > MAX_PASSWORD_LENGTH {
            error = true;
            field_errors.insert(
                "password".to_string(),
                vec![format!(
                    "password must be at least {min} characters and no loner than {max} characters",
                    min = MIN_PASSWORD_LENGTH,
                    max = MAX_PASSWORD_LENGTH
                )],
            );
        };
        return match error {
            true => Err(ServerError::ValidationError(field_errors)),
            false => Ok(UserInput {
                username: value.username,
                password: value.password,
            }),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::features::user::{dto::UserInputRequest, model::UserInput};

    #[test]
    fn test_valid_username_and_password() {
        // Arrange
        let expected_username = "test".to_string();
        let expected_password = "password".to_string();
        let expected_register_user = UserInput {
            username: expected_username.clone(),
            password: expected_password.clone(),
        };
        let expected_register_user_request = UserInputRequest {
            username: expected_username.clone(),
            password: expected_password.clone(),
        };
        // Act
        let actual_register_user: UserInput =
            UserInput::try_from(expected_register_user_request).unwrap();
        // Assert
        assert_eq!(actual_register_user, expected_register_user);
    }
}
