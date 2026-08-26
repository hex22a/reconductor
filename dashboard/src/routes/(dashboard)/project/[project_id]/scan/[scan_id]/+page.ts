import { get_scan_details } from '@/lib/services/scans';
import { get } from 'svelte/store';
import type { PageLoad } from './$types';
import { csrf } from '@/lib/stores/csrf';
import { isError } from '@/lib/transport/ErrorResponse';
import { error } from '@sveltejs/kit';
import { list } from '@/lib/services/runs';

export const load: PageLoad = async ({ parent, params, fetch }) => {
    await parent();

    const csrfToken = get(csrf);
    const scan_details = await get_scan_details(
        fetch,
        csrfToken,
        params.project_id,
        params.scan_id,
    );
    if (isError(scan_details)) {
        console.error(scan_details);
        throw error(404, 'Failed to load scan details');
    }
    const runs = await list(fetch, csrfToken, params.project_id, params.scan_id);
    if (isError(runs)) {
        console.error(runs);
        throw error(404, 'Failed to load scan runs');
    }
    return { scan_details, runs };
};
