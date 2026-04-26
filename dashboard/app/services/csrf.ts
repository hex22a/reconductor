import { API_CSRF_URL } from '~/constants';

export function fetchCsrf() {
    return fetch(API_CSRF_URL, { credentials: 'include' });
}
