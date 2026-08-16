import { create, list } from '@/lib/services/scans';
import { csrf } from '@/lib/stores/csrf';
import { isError, type ErrorResponse } from '@/lib/transport/ErrorResponse';
import type { Scan } from '@/lib/transport/Scan';
import { get } from 'svelte/store';

let scans = $state<Array<Scan>>([]);

export const scansStore = {
    get scans() {
        return scans;
    },
    async add(target: string, schedule: string): Promise<void | ErrorResponse> {
        const csrfToken = get(csrf);
        const response = await create(csrfToken, target, schedule);
        if (isError(response)) {
            return response;
        }
        scans.push(response);
    },
    async list(after?: string): Promise<void | ErrorResponse> {
        const csrfToken = get(csrf);
        const response = await list(csrfToken, after);
        if (isError(response)) {
            return response;
        }
        scans = scans.concat(response.data);
    },
};
