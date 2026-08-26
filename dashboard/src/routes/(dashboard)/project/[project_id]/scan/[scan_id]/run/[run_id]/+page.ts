import { get } from 'svelte/store';
import type { PageLoad } from './$types';
import { csrf } from '@/lib/stores/csrf';
import { isError } from '@/lib/transport/ErrorResponse';
import { error } from '@sveltejs/kit';
import { get_run_details } from '@/lib/services/runs';
import { list } from '@/lib/services/hosts';

export const load: PageLoad = async ({ parent, params, fetch }) => {
    await parent();

    const csrfToken = get(csrf);
    const run_details = await get_run_details(
        fetch,
        csrfToken,
        params.project_id,
        params.scan_id,
        params.run_id,
    );
    if (isError(run_details)) {
        console.error(run_details);
        throw error(404, 'Failed to load scan run details');
    }
    const hosts = await list(fetch, csrfToken, params.project_id, params.scan_id, params.run_id);
    if (isError(hosts)) {
        console.error(hosts);
        throw error(404, 'Failed to load hosts');
    }
    console.log(hosts);
    return { run_details, hosts };
};
