import { isError } from '@/lib/transport/ErrorResponse';
import { csrf } from '@/lib/stores/csrf';
import { get } from 'svelte/store';
import { get_project_details } from '@/lib/services/projects';
import type { PageLoad } from './$types';
import { list } from '@/lib/services/scans';
import { error } from '@sveltejs/kit';

export const load: PageLoad = async ({ parent, params, fetch }) => {
    await parent();

    const csrfToken = get(csrf);
    const project = await get_project_details(fetch, csrfToken, params.project_id);
    if (isError(project)) {
        console.error(project);
        throw error(404, 'Failed to load projects');
    }
    const scans = await list(fetch, csrfToken, params.project_id);
    if (isError(scans)) {
        console.error(scans);
        throw error(404, 'Failed to load scans');
    }
    return { project, scans };
};
