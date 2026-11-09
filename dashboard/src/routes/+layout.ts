import { auth } from '#src/lib/stores/auth';
import { csrf } from '#src/lib/stores/csrf';

export const ssr = false;

export async function load() {
    await csrf.fetchCsrf();
    await auth.fetchMe();
}
