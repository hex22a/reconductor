use crate::domain::{
    secret_types::{CsrfCookie, CsrfToken, SessionToken},
    types::{UserId, Username},
};

#[derive(Debug, Clone)]
pub struct UserSession {
    pub token: SessionToken,
    pub user_id: UserId,
    pub username: Username,
    pub csrf_token: CsrfToken,
    pub csrf_cookie: CsrfCookie,
}
