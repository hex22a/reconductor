use uuid::Uuid;

use crate::domain::types::{CsrfCookie, CsrfToken, SessionToken, Username};

#[derive(Debug, Clone, PartialEq)]
pub struct UserSession {
    pub token: SessionToken,
    pub user_id: Uuid,
    pub username: Username,
    pub csrf_token: CsrfToken,
    pub csrf_cookie: CsrfCookie,
}
