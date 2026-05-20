import { auth } from '@/lib/stores/auth';
import { csrf } from '@/lib/stores/csrf';

export const ssr = false;

export async function load() {
    await csrf.fetchCsrf();
    await auth.fetchMe();
}
