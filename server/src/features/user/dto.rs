use serde::{Deserialize, Serialize};

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
