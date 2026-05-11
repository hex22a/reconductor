import { API_LOGIN_URL, API_LOGOUT_URL, API_ME_URL, API_REGISTER_URL } from '@/constants';

export async function signUp(username?: string, password?: string) {
    return fetch(API_REGISTER_URL, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ username, password }),
    });
}

export async function signIn(csrfToken: string | null, username?: string, password?: string) {
    return fetch(API_LOGIN_URL, {
        method: 'POST',
        credentials: 'include',
        headers: {
            'Content-Type': 'application/json',
            'X-CSRF-Token': csrfToken ?? '',
        },
        body: JSON.stringify({ username, password }),
    });
}

export async function logout() {
    return fetch(API_LOGOUT_URL, {
        method: 'POST',
        credentials: 'include',
    });
}

export async function fetchMe(): Promise<string> {
    const res = await fetch(API_ME_URL, { credentials: 'include' });
    const data = await res.json();
    return data.username;
}
