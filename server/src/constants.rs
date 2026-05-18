pub(crate) const API_HEALTH_ENDPOINT_V1: &str = "/api/v1/health";
pub(crate) const API_REGISTER_ENDPOINT_V1: &str = "/api/v1/register";
pub(crate) const API_LOGIN_ENDPOINT_V1: &str = "/api/v1/login";
pub(crate) const API_LOGOUT_ENDPOINT_V1: &str = "/api/v1/logout";
pub(crate) const API_CSRF_ENDPOINT_V1: &str = "/api/v1/csrf";
pub(crate) const API_ME_ENDPOINT_V1: &str = "/api/v1/me";
pub(crate) const API_PROJECTS_ENDPOINT_V1: &str = "/api/v1/projects";

pub(crate) const CSRF_HEADER: &str = "X-CSRF-Token";

pub(crate) const USER_SESSION_PREFIX: &str = "user_session";
pub(crate) const ANONYMOUS_CSRF_PREFIX: &str = "csrf_anonymous";

pub(crate) const USER_SESSION_COOKIE_NAME: &str = "recon_user_session";
pub(crate) const CSRF_COOKIE_NAME: &str = "recon_csrf";

pub(crate) const USER_SESSION_TTL_SECONDS: u64 = 15 * 60;
pub(crate) const ANONYMOUS_CSRF_TTL_SECONDS: u64 = 5 * 60;

pub(crate) const NONCE_SIZE_BYTES: usize = 64;
pub(crate) const SESSION_ID_SIZE_BYTES: usize = 32;

pub(crate) const REDIS_POOL_SIZE: usize = 2;

pub(crate) const PROJECTS_PAGE_SIZE_LIMIT: i64 = 15;

// Password setting https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#argon2id
pub(crate) const PASSWORD_MEMORY_COST_BYTES: u32 = 46 * 1024;
pub(crate) const PASSWORD_TIME_COST_PASSES: u32 = 1;
pub(crate) const PASSWORD_PARALLELISM: u32 = 1;

pub(crate) const MIN_USERNAME_LENGTH: usize = 4;
pub(crate) const MAX_USERNAME_LENGTH: usize = 32;
pub(crate) const MIN_PASSWORD_LENGTH: usize = 6;
pub(crate) const MAX_PASSWORD_LENGTH: usize = 74;

pub(crate) const UNEXPECTED_ERROR_MESSAGE: &str = "Unexpected error";
pub(crate) const UNAUTHORIZED_ERROR_MESSAGE: &str = "Unauthorized";
