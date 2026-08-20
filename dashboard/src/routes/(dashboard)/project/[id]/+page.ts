import { isError } from '@/lib/transport/ErrorResponse';
import { csrf } from '@/lib/stores/csrf';
import { get } from 'svelte/store';
import { get_project_details } from '@/lib/services/projects';
import type { PageLoad } from './$types';
import { scansStore } from '@/lib/components/scans/scans.svelte';

export const load: PageLoad = async ({ parent, params }) => {
    await parent();

    const csrfToken = get(csrf);
    const project_details = await get_project_details(csrfToken, params.id);
    if (isError(project_details)) {
        console.error(project_details);
    }
    await scansStore.list(params.id);
    return project_details;
};
