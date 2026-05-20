import { API_PROJECTS_URL, NETWORK_ERROR_CODE, NETWORK_ERROR_MESSAGE } from '@/constants';
import type { ErrorResponse } from '../transport/ErrorResponse';
import type { Page } from '../transport/Pagination';
import type { Project } from '../transport/Project';

export async function create(
    csrfToken: string | null,
    name: string,
): Promise<Project | ErrorResponse> {
    try {
        const res = await fetch(API_PROJECTS_URL, {
            method: 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
                'X-CSRF-Token': csrfToken ?? '',
            },
            body: JSON.stringify({ name }),
        });
        if (!res.ok) {
            return (await res.json()) as ErrorResponse;
        }
        return (await res.json()) as Project;
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
): Promise<Page<Project> | ErrorResponse> {
    try {
        const res = await fetch(API_PROJECTS_URL, {
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
        return (await res.json()) as Page<Project>;
    } catch {
        return {
            code: NETWORK_ERROR_CODE,
            error: NETWORK_ERROR_MESSAGE,
        };
    }
}
