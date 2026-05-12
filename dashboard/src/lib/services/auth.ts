import {
    API_LOGIN_URL,
    API_LOGOUT_URL,
    API_ME_URL,
    API_REGISTER_URL,
    NETWORK_ERROR_CODE,
    NETWORK_ERROR_MESSAGE,
} from '@/constants';
import type { ErrorResponse } from '../transport/ErrorResponse';
import type { LoginResponse, MeResponse } from '../transport/Auth';

export async function signUp(username?: string, password?: string): Promise<null | ErrorResponse> {
    try {
        const res = await fetch(API_REGISTER_URL, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username, password }),
        });
        if (!res.ok) {
            return (await res.json()) as ErrorResponse;
        }
        return null;
    } catch {
        return {
            code: NETWORK_ERROR_CODE,
            error: NETWORK_ERROR_MESSAGE,
        };
    }
}

export async function signIn(
    csrfToken: string | null,
    username?: string,
    password?: string,
): Promise<LoginResponse | ErrorResponse> {
    try {
        const res = await fetch(API_LOGIN_URL, {
            method: 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
                'X-CSRF-Token': csrfToken ?? '',
            },
            body: JSON.stringify({ username, password }),
        });
        if (!res.ok) {
            return (await res.json()) as ErrorResponse;
        }
        return (await res.json()) as LoginResponse;
    } catch {
        return {
            code: NETWORK_ERROR_CODE,
            error: NETWORK_ERROR_MESSAGE,
        };
    }
}

export async function logout(): Promise<null | ErrorResponse> {
    try {
        const res = await fetch(API_LOGOUT_URL, {
            method: 'POST',
            credentials: 'include',
        });
        if (!res.ok) {
            return (await res.json()) as ErrorResponse;
        }
        return null;
    } catch {
        return {
            code: NETWORK_ERROR_CODE,
            error: NETWORK_ERROR_MESSAGE,
        };
    }
}

export async function fetchMe(): Promise<MeResponse | ErrorResponse> {
    try {
        const res = await fetch(API_ME_URL, { credentials: 'include' });
        if (!res.ok) {
            return (await res.json()) as ErrorResponse;
        }
        return (await res.json()) as MeResponse;
    } catch {
        return {
            code: NETWORK_ERROR_CODE,
            error: NETWORK_ERROR_MESSAGE,
        };
    }
}
