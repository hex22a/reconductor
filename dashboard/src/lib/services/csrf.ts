import { API_CSRF_URL, NETWORK_ERROR_CODE, NETWORK_ERROR_MESSAGE } from '@/constants';
import type { CsrfResponse } from '../transport/CsrfResponse';
import type { ErrorResponse } from '../transport/ErrorResponse';

export async function fetchCsrf(): Promise<CsrfResponse | ErrorResponse> {
    try {
        const res = await fetch(API_CSRF_URL, { credentials: 'include' });
        if (!res.ok) {
            return (await res.json()) as ErrorResponse;
        }
        return (await res.json()) as CsrfResponse;
    } catch {
        return {
            code: NETWORK_ERROR_CODE,
            error: NETWORK_ERROR_MESSAGE,
        };
    }
}
