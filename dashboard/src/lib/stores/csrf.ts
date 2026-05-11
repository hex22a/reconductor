import { writable } from 'svelte/store';
import { fetchCsrf } from '@/lib/services/csrf';

function createCsrfStore() {
    const { subscribe, set } = writable<string | null>(null);

    return {
        subscribe,
        fetchCsrf: async () => {
            const csrf = await fetchCsrf();
            set(csrf);
        },
    };
}

export const csrf = createCsrfStore();
