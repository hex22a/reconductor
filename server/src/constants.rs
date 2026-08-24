pub const API_HEALTH_ENDPOINT_V1: &str = "/api/v1/health";
pub const API_REGISTER_ENDPOINT_V1: &str = "/api/v1/register";
pub const API_LOGIN_ENDPOINT_V1: &str = "/api/v1/login";
pub const API_LOGOUT_ENDPOINT_V1: &str = "/api/v1/logout";
pub const API_CSRF_ENDPOINT_V1: &str = "/api/v1/csrf";
pub const API_ME_ENDPOINT_V1: &str = "/api/v1/me";
pub const API_PROJECTS_ENDPOINT_V1: &str = "/api/v1/projects";
pub const API_PROJECT_SCANS_ENDPOINT_V1: &str = "/scans";
pub const API_SCAN_RUNS_ENDPOINT_V1: &str = "/runs";
pub const API_HOSTS_ENDPOINT_V1: &str = "/hosts";
pub const API_PORTS_ENDPOINT_V1: &str = "/ports";

pub const CSRF_HEADER: &str = "X-CSRF-Token";

pub const USER_SESSION_PREFIX: &str = "user_session";
pub const ANONYMOUS_CSRF_PREFIX: &str = "csrf_anonymous";

pub const USER_SESSION_COOKIE_NAME: &str = "recon_user_session";
pub const CSRF_COOKIE_NAME: &str = "recon_csrf";

pub const SCANS_QUEUE: &str = "scans";

pub const USER_SESSION_TTL_SECONDS: u64 = 15 * 60;
pub const ANONYMOUS_CSRF_TTL_SECONDS: u64 = 5 * 60;

pub const NONCE_SIZE_BYTES: usize = 64;
pub const SESSION_ID_SIZE_BYTES: usize = 32;

pub const REDIS_POOL_SIZE: usize = 2;

pub const PROJECTS_PAGE_SIZE_LIMIT: i64 = 15;
pub const SCANS_PAGE_SIZE_LIMIT: i64 = 15;
pub const SCAN_RUNS_PAGE_SIZE_LIMIT: i64 = 15;
pub const HOSTS_PAGE_SIZE_LIMIT: i64 = 15;
pub const PORTS_PAGE_SIZE_LIMIT: i64 = 15;

// Password setting https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#argon2id
pub const PASSWORD_MEMORY_COST_BYTES: u32 = 46 * 1024;
pub const PASSWORD_TIME_COST_PASSES: u32 = 1;
pub const PASSWORD_PARALLELISM: u32 = 1;

pub const MIN_USERNAME_LENGTH: usize = 4;
pub const MAX_USERNAME_LENGTH: usize = 32;
pub const MIN_PASSWORD_LENGTH: usize = 6;
pub const MAX_PASSWORD_LENGTH: usize = 74;

pub const UNEXPECTED_ERROR_MESSAGE: &str = "Unexpected error";
pub const UNAUTHORIZED_ERROR_MESSAGE: &str = "Unauthorized";
