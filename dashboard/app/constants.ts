const DEFAULT_API_URL = 'http://localhost:4000';

export const API_URL: string = import.meta.env.VITE_API_URL || DEFAULT_API_URL;

export const API_REGISTER_URL = `${API_URL}/api/v1/register`;
export const API_LOGIN_URL = `${API_URL}/api/v1/login`;
export const API_ME_URL = `${API_URL}/api/v1/me`;
export const API_LOGOUT_URL = `${API_URL}/api/v1/logout`;
export const GRAPHQL_URL = `${API_URL}/graphql`;
