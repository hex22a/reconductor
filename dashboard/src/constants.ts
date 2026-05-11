import { PUBLIC_API_URL } from '$env/static/public';

const DEFAULT_API_URL = 'http://localhost:3000';
const API_URL = PUBLIC_API_URL || DEFAULT_API_URL;

export const API_REGISTER_URL = `${API_URL}/api/v1/register`;
export const API_LOGIN_URL = `${API_URL}/api/v1/login`;
export const API_ME_URL = `${API_URL}/api/v1/me`;
export const API_LOGOUT_URL = `${API_URL}/api/v1/logout`;
export const API_CSRF_URL = `${API_URL}/api/v1/csrf`;
