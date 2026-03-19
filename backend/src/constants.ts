const DEFAULT_DASHBOARD_URL = 'http://localhost:5173';

export const DASHBOARD_URL: string =
    (process.env.NODE_ENV === 'production' ? process.env.DASHBOARD_URL : DEFAULT_DASHBOARD_URL) ||
    DEFAULT_DASHBOARD_URL;

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
