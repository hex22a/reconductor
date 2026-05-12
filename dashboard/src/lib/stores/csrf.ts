import { writable } from 'svelte/store';
import { fetchCsrf } from '@/lib/services/csrf';
import { isError, type ErrorResponse } from '../transport/ErrorResponse';

function createCsrfStore() {
    const { subscribe, set } = writable<string | null>(null);

    return {
        subscribe,
        fetchCsrf: async (): Promise<void | ErrorResponse> => {
            const csrfResponse = await fetchCsrf();
            if (isError(csrfResponse)) {
                return csrfResponse;
            }
            set(csrfResponse.csrf_token);
        },
        updateCsrf: (csrfToken: string) => {
            set(csrfToken);
        },
    };
}

export const csrf = createCsrfStore();
