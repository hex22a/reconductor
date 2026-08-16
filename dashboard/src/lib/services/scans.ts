import { API_SCANS_URL, NETWORK_ERROR_CODE, NETWORK_ERROR_MESSAGE } from '@/constants';
import type { Scan } from '../transport/Scan';
import type { ErrorResponse } from '../transport/ErrorResponse';
import type { Page } from '../transport/Pagination';

export async function create(
    csrfToken: string | null,
    project_id: string,
    target: string,
    schedule: string,
): Promise<Scan | ErrorResponse> {
    try {
        const res = await fetch(API_SCANS_URL, {
            method: 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
                'X-CSRF-Token': csrfToken ?? '',
            },
            body: JSON.stringify({ project_id, target, schedule }),
        });
        if (!res.ok) {
            return (await res.json()) as ErrorResponse;
        }
        return (await res.json()) as Scan;
    } catch {
        return {
            code: NETWORK_ERROR_CODE,
            error: NETWORK_ERROR_MESSAGE,
        };
    }
}

export async function get_project_details(
    csrfToken: string | null,
    scanId: string,
): Promise<Scan | ErrorResponse> {
    try {
        const res = await fetch(`${API_SCANS_URL}/${scanId}`, {
            method: 'GET',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
                'X-CSRF-Token': csrfToken ?? '',
            },
        });
        if (!res.ok) {
            return (await res.json()) as ErrorResponse;
        }
        return (await res.json()) as Scan;
    } catch {
        return {
            code: NETWORK_ERROR_CODE,
            error: NETWORK_ERROR_MESSAGE,
        };
    }
}

export async function list(
    csrfToken: string | null,
    after?: string,
): Promise<Page<Scan> | ErrorResponse> {
    try {
        const res = await fetch(API_SCANS_URL, {
            method: 'GET',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
                'X-CSRF-Token': csrfToken ?? '',
            },
            body: after ? JSON.stringify({ after }) : null,
        });
        if (!res.ok) {
            return (await res.json()) as ErrorResponse;
        }
        return (await res.json()) as Page<Scan>;
    } catch {
        return {
            code: NETWORK_ERROR_CODE,
            error: NETWORK_ERROR_MESSAGE,
        };
    }
}
