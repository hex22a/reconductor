import { API_LOGIN_URL, API_REGISTER_URL } from '~/constants';

export function signUp(username?: string, password?: string) {
    return fetch(API_REGISTER_URL, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ username, password }),
    });
}

export function signIn(username?: string, password?: string) {
    return fetch(API_LOGIN_URL, {
        method: 'POST',
        credentials: 'include',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ username, password }),
    });
}
