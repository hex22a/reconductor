import { csrf } from '@/lib/stores/csrf';
import { isError } from '@/lib/transport/ErrorResponse';
import type { PageLoad } from './$types';
import { list } from '@/lib/services/projects';
import { get } from 'svelte/store';

export const load: PageLoad = async ({ parent, fetch }) => {
    await parent();
    const csrfToken = get(csrf);
    const res = await list(fetch, csrfToken);
    if (isError(res)) {
        console.error(res);
    }
    return res;
};
