import { writable, get } from 'svelte/store';
import { logout, fetchMe, signIn, signUp } from '@/lib/services/auth';
import { csrf } from './csrf';
import { isError, type ErrorResponse } from '../transport/ErrorResponse';

function createAuthStore() {
    const { subscribe, set } = writable<string | null>(null);

    return {
        subscribe,
        register: async (username: string, password: string): Promise<void | ErrorResponse> => {
            const signupResponse = await signUp(username, password);
            if (signupResponse) {
                return signupResponse;
            }
        },
        login: async (username: string, password: string): Promise<void | ErrorResponse> => {
            let csrfToken = get(csrf);
            let signInResponse = await signIn(csrfToken, username, password);
            if (isError(signInResponse)) {
                return signInResponse;
            }
            csrf.updateCsrf(signInResponse.csrf_token);
            set(username);
        },
        logout: async (): Promise<void | ErrorResponse> => {
            await logout();
            set(null);
        },
        fetchMe: async (): Promise<void | ErrorResponse> => {
            const meResponse = await fetchMe();
            if (isError(meResponse)) {
                return meResponse;
            }
            set(meResponse.username);
        },
    };
}

export const auth = createAuthStore();
