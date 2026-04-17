const DEFAULT_DASHBOARD_URL = 'http://localhost:5173';
const DEFAULT_RABBITMQ_URL = 'amqp://reconductor:reconductor@rabbitmq:5672';

export const DASHBOARD_URL: string =
    (process.env.NODE_ENV === 'production' ? process.env.DASHBOARD_URL : DEFAULT_DASHBOARD_URL) ||
    DEFAULT_DASHBOARD_URL;
export const RABBITMQ_URL: string =
    (process.env.NODE_ENV === 'production' ? process.env.RABBITMQ_URL : DEFAULT_RABBITMQ_URL) ||
    DEFAULT_RABBITMQ_URL;

export const SCAN_QUEUE = 'scans';

export const ACCESS_CONTROL_ALLOW_ORIGIN_HEADER = 'access-control-allow-origin';
export const ACCESS_CONTROL_ALLOW_METHODS_HEADER = 'access-control-allow-methods';
export const ACCESS_CONTROL_ALLOW_HEADERS_HEADER = 'access-control-allow-headers';
export const ACCESS_CONTROL_ALLOW_CREDENTIALS_HEADER = 'access-control-allow-credentials';
export const CORS_ALLOWED_METHODS = 'GET,POST,OPTIONS';
export const CORS_ALLOWED_HEADERS = 'Content-Type';
export const CORS_ALLOW_CREDENTIALS = 'true';
export const CORS_HEADERS = {
    [ACCESS_CONTROL_ALLOW_ORIGIN_HEADER]: DASHBOARD_URL,
    [ACCESS_CONTROL_ALLOW_METHODS_HEADER]: CORS_ALLOWED_METHODS,
    [ACCESS_CONTROL_ALLOW_HEADERS_HEADER]: CORS_ALLOWED_HEADERS,
    [ACCESS_CONTROL_ALLOW_CREDENTIALS_HEADER]: CORS_ALLOW_CREDENTIALS,
};

export const API_REGISTER_ENDPOINT_V1 = '/api/v1/register';
export const API_LOGIN_ENDPOINT_V1 = '/api/v1/login';
export const API_ME_ENDPOINT_V1 = '/api/v1/me';
export const API_LOGOUT_ENDPOINT_V1 = '/api/v1/logout';
export const API_HEALTH_ENDPOINT_V1 = '/api/v1/health';
export const GRAPHQL_ENDPOINT = '/graphql';

export const MIN_USERNAME_LENGTH = 4;
export const MAX_USERNAME_LENGTH = 32;
export const MIN_PASSWORD_LENGTH = 6;
export const MAX_PASSWORD_LENGTH = 74;
export const Z_REGISTER_SCHEMA_ERROR_MESSAGE = 'wrong data';
export const Z_USERNAME_STRING_ERROR_MESSAGE = 'username must be a string';
export const Z_PASSWORD_STRING_ERROR_MESSAGE = 'password must be a string';
export const Z_USERNAME_LENGTH_ERROR_MESSAGE = `username must be at least ${MIN_USERNAME_LENGTH} characters and no longer than ${MAX_USERNAME_LENGTH} characters`;
export const Z_PASSWORD_LENGTH_ERROR_MESSAGE = `password must be at least ${MIN_PASSWORD_LENGTH} characters and no longer than ${MAX_PASSWORD_LENGTH} characters`;
export const Z_SCAN_TARGET_ERROR_MESSAGE =
    'target must be either IPv4, IPv6, CIDR IPv4 or CIDR IPv6';
export const Z_SCAN_PROJECT_ID_ERROR_MESSAGE = 'project id must be uuid v7';
export const Z_SCAN_SHCEDULE_ERROR_MESSAGE = 'schedule must be a cron expression';
export const Z_SCAN_SCHEMA_ERROR_MESSAGE = 'target and project id required';

export const UNEXPECTED_END_OF_JSON_ERROR_MESSAGE = 'Unexpected end of JSON input';
export const UNEXPECTED_ERROR_MESSAGE = 'Unexpected error';
export const DATABASE_ERROR_MESSAGE = 'Something went wrong';

export const USER_SESSION_COOKIE_NAME = 'recon_user_session';
export const USER_SESSION_TTL_SECONDS = 15 * 60;
export const USER_SESSION_PREFIX = 'user_session';

export const TOKEN_RANDOM_BYTES_ARRAY_LENGTH = 32;

export const GRAPHQL_UNAUTHORIZED_ERROR_MESSAGE = 'Unauthorized';
export const GRAPHQL_ERROR_EXTENSION_CODE = 'FORBIDDEN';

export const UNAUTHORIZED_ERROR_MESSAGE = 'Unauthorized';

export const PROJECTS_PAGE_SIZE = 15;
export const SCANS_PAGE_SIZE = 25;
export const SCAN_RUNS_PAGE_SIZE = 35;
