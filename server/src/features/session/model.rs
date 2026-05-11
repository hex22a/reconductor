use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct UserSession {
    pub token: String,
    pub user_id: Uuid,
    pub username: String,
    pub csrf_token: String,
}
