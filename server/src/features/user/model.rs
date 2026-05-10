use std::collections::HashMap;

use crate::{
    constants::{
        MAX_PASSWORD_LENGTH, MAX_USERNAME_LENGTH, MIN_PASSWORD_LENGTH, MIN_USERNAME_LENGTH,
    },
    domain::error::ServerError,
    features::user::dto::RegisterUserRequest,
};

#[derive(Debug, PartialEq)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}

impl TryFrom<RegisterUserRequest> for RegisterUser {
    type Error = ServerError;

    fn try_from(value: RegisterUserRequest) -> Result<Self, Self::Error> {
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
            false => Ok(RegisterUser {
                username: value.username,
                password: value.password,
            }),
        };
    }
}

#[derive(Debug, PartialEq)]
pub struct AuthSession {
    pub session_id: String,
    pub csrf_token: String,
}

#[cfg(test)]
mod tests {
    use crate::features::user::{dto::RegisterUserRequest, model::RegisterUser};

    #[test]
    fn test_valid_username_and_password() {
        // Arrange
        let expected_username = "test".to_string();
        let expected_password = "password".to_string();
        let expected_register_user = RegisterUser {
            username: expected_username.clone(),
            password: expected_password.clone(),
        };
        let expected_register_user_request = RegisterUserRequest {
            username: expected_username.clone(),
            password: expected_password.clone(),
        };
        // Act
        let actual_register_user: RegisterUser =
            RegisterUser::try_from(expected_register_user_request).unwrap();
        // Assert
        assert_eq!(actual_register_user, expected_register_user);
    }
}
