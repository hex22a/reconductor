import { writable, get } from 'svelte/store';
import { logout, fetchMe, signIn, signUp } from '@/lib/services/auth';
import { csrf } from './csrf';

function createAuthStore() {
    const { subscribe, set } = writable<string | null>(null);

    return {
        subscribe,
        register: async (username: string, password: string) => {
            await signUp(username, password);
        },
        login: async (username: string, password: string) => {
            let csrfToken = get(csrf);
            await signIn(csrfToken, username, password);
            set(username);
        },
        logout: async () => {
            await logout();
            set(null);
        },
        fetchMe: async () => {
            const username = await fetchMe();
            set(username);
        },
    };
}

export const auth = createAuthStore();
