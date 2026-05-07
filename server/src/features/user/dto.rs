use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub password: String,
}
