pub const API_HEALTH_ENDPOINT_V1: &str = "/api/v1/health";

pub const USER_SESSION_PREFIX: &str = "user_session";
pub const ANONYMOUS_CSRF_PREFIX: &str = "csrf_anonymous";

pub const USER_SESSION_TTL_SECONDS: u64 = 15 * 60;
pub const ANONYMOUS_CSRF_TTL_SECONDS: u64 = 5 * 60;

pub const NONCE_SIZE_BYTES: usize = 64;
pub const SESSION_COOKIE_SIZE_BYTES: usize = 32;

// Password setting https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#argon2id
pub const PASSWORD_MEMORY_COST_BYTES: u32 = 46 * 1024;
pub const PASSWORD_TIME_COST_PASSES: u32 = 1;
pub const PASSWORD_PARALLELISM: u32 = 1;
