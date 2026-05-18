import { writable, get } from 'svelte/store';
import { logout, fetchMe, login, register } from '@/lib/services/auth';
import { csrf } from './csrf';
import { isError, type ErrorResponse } from '../transport/ErrorResponse';

type AuthState = {
    user: string | null;
    ready: boolean;
};

function createAuthStore() {
    const { subscribe, set } = writable<AuthState>({ user: null, ready: false });

    return {
        subscribe,
        signUp: async (username: string, password: string): Promise<void | ErrorResponse> => {
            const signupResponse = await register(username, password);
            if (signupResponse) {
                return signupResponse;
            }
        },
        signIn: async (username: string, password: string): Promise<void | ErrorResponse> => {
            let csrfToken = get(csrf);
            let signInResponse = await login(csrfToken, username, password);
            if (isError(signInResponse)) {
                return signInResponse;
            }
            csrf.updateCsrf(signInResponse.csrf_token);
            set({ user: username, ready: true });
        },
        logout: async (): Promise<void | ErrorResponse> => {
            await logout();
            set({ user: null, ready: true });
        },
        fetchMe: async (): Promise<void | ErrorResponse> => {
            const meResponse = await fetchMe();
            if (isError(meResponse)) {
                set({ user: null, ready: true });
                return meResponse;
            }
            set({ user: meResponse.username, ready: true });
        },
    };
}

export const auth = createAuthStore();
