import {
    API_HOSTS_URL,
    API_PROJECTS_URL,
    API_RUNS_URL,
    API_SCANS_URL,
    NETWORK_ERROR_CODE,
    NETWORK_ERROR_MESSAGE,
} from '@/constants';
import type { ErrorResponse } from '../transport/ErrorResponse';
import type { Page } from '../transport/Pagination';
import type { SvelteKitFetch } from './svelte';
import type { Host } from '../transport/Host';

export async function get_host_details(
    fetch: SvelteKitFetch,
    csrfToken: string | null,
    projectId: string,
    scanId: string,
    runId: string,
    hostId: string,
): Promise<Host | ErrorResponse> {
    try {
        const res = await fetch(
            `${API_PROJECTS_URL}/${projectId}/${API_SCANS_URL}/${scanId}/${API_RUNS_URL}/${runId}/${API_HOSTS_URL}/${hostId}`,
            {
                method: 'GET',
                credentials: 'include',
                headers: {
                    'Content-Type': 'application/json',
                    'X-CSRF-Token': csrfToken ?? '',
                },
            },
        );
        if (!res.ok) {
            return (await res.json()) as ErrorResponse;
        }
        return (await res.json()) as Host;
    } catch {
        return {
            code: NETWORK_ERROR_CODE,
            error: NETWORK_ERROR_MESSAGE,
        };
    }
}

export async function list(
    fetch: SvelteKitFetch,
    csrfToken: string | null,
    projectId: string,
    scanId: string,
    runId: string,
    after?: string,
): Promise<Page<Host> | ErrorResponse> {
    try {
        const res = await fetch(
            `${API_PROJECTS_URL}/${projectId}/${API_SCANS_URL}/${scanId}/${API_RUNS_URL}/${runId}/${API_HOSTS_URL}`,
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
        return (await res.json()) as Page<Host>;
    } catch {
        return {
            code: NETWORK_ERROR_CODE,
            error: NETWORK_ERROR_MESSAGE,
        };
    }
}
