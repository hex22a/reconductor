const DEFAULT_API_URL = 'http://localhost:4000';

export const API_URL: string = process.env.API_URL || DEFAULT_API_URL;

export const API_REGISTER_URL = `${API_URL}/api/v1/register`;
export const API_LOGIN_URL = `${API_URL}/api/v1/login`;
