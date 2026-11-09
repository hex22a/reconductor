import { csrf } from '#src/lib/stores/csrf';
import { isError } from '#src/lib/transport/ErrorResponse';
import type { PageLoad } from './$types';
import { list } from '#src/lib/services/projects';
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
