import {
    API_HOSTS_URL,
    API_PORTS_URL,
    API_PROJECTS_URL,
    API_RUNS_URL,
    API_SCANS_URL,
    NETWORK_ERROR_CODE,
    NETWORK_ERROR_MESSAGE,
} from '@/constants';
import type { ErrorResponse } from '../transport/ErrorResponse';
import type { Page } from '../transport/Pagination';
import type { SvelteKitFetch } from './svelte';
import type { Port } from '../transport/Port';

export async function list(
    fetch: SvelteKitFetch,
    csrfToken: string | null,
    projectId: string,
    scanId: string,
    runId: string,
    hostId: string,
    after?: string,
): Promise<Page<Port> | ErrorResponse> {
    try {
        const res = await fetch(
            `${API_PROJECTS_URL}/${projectId}/${API_SCANS_URL}/${scanId}/${API_RUNS_URL}/${runId}/${API_HOSTS_URL}/${hostId}/${API_PORTS_URL}`,
            {
                method: 'GET',
                credentials: 'include',
                headers: {
                    'Content-Type': 'application/json',
                    'X-CSRF-Token': csrfToken ?? '',
                },
                body: after ? JSON.stringify({ after }) : null,
            },
        );
        if (!res.ok) {
            return (await res.json()) as ErrorResponse;
        }
        return (await res.json()) as Page<Port>;
    } catch {
        return {
            code: NETWORK_ERROR_CODE,
            error: NETWORK_ERROR_MESSAGE,
        };
    }
}
