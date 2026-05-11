import { writable } from 'svelte/store';
import { logout, fetchMe, signIn } from '@/lib/services/auth';

function createAuthStore() {
    const { subscribe, set } = writable<string | null>(null);

    return {
        subscribe,
        login: async (username: string, password: string) => {
            await signIn(username, password);
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
