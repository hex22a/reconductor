import { get } from 'svelte/store';
import type { PageLoad } from './$types';
import { csrf } from '@/lib/stores/csrf';
import { isError } from '@/lib/transport/ErrorResponse';
import { error } from '@sveltejs/kit';
import { get_host_details } from '@/lib/services/hosts';
import { list } from '@/lib/services/ports';

export const load: PageLoad = async ({ parent, params, fetch }) => {
    await parent();

    const csrfToken = get(csrf);
    const host_details = await get_host_details(
        fetch,
        csrfToken,
        params.project_id,
        params.scan_id,
        params.run_id,
        params.host_id,
    );
    if (isError(host_details)) {
        console.error(host_details);
        throw error(404, 'Failed to load host details');
    }
    const ports = await list(
        fetch,
        csrfToken,
        params.project_id,
        params.scan_id,
        params.run_id,
        params.host_id,
    );
    if (isError(ports)) {
        console.error(ports);
        throw error(404, 'Failed to load ports');
    }
    return { host_details, ports };
};
