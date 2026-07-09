import { isError } from '@/lib/transport/ErrorResponse';
import { csrf } from '@/lib/stores/csrf';
import { get } from 'svelte/store';
import { get_project_details } from '@/lib/services/projects';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
    await parent();

    const csrfToken = get(csrf);
    const res = await get_project_details(csrfToken, params.id);
    if (isError(res)) {
        console.error(res);
    }
    return res;
};
