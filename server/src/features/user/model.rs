use std::collections::HashMap;

use crate::{
    constants::{
        MAX_PASSWORD_LENGTH, MAX_USERNAME_LENGTH, MIN_PASSWORD_LENGTH, MIN_USERNAME_LENGTH,
    },
    domain::error::ServerError,
    features::user::dto::RegisterUserRequest,
};

struct RegisterUser {
    username: String,
    password: String,
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
