import { API_CSRF_URL } from '@/constants';

export async function fetchCsrf(): Promise<string> {
    const res = await fetch(API_CSRF_URL, { credentials: 'include' });
    const data = await res.json();
    return data.csrf_token;
}
