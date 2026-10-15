use uuid::Uuid;

use crate::domain::{
    csrf_cookie::CsrfCookie, csrf_token::CsrfToken, session_token::SessionToken, username::Username,
};

#[derive(Debug, Clone, PartialEq)]
pub struct UserSession {
    pub token: SessionToken,
    pub user_id: Uuid,
    pub username: Username,
    pub csrf_token: CsrfToken,
    pub csrf_cookie: CsrfCookie,
}
