import { get_scan_details } from '@/lib/services/scans';
import { get } from 'svelte/store';
import type { PageLoad } from './$types';
import { csrf } from '@/lib/stores/csrf';
import { isError } from '@/lib/transport/ErrorResponse';

export const load: PageLoad = async ({ parent, params }) => {
    await parent();

    const csrfToken = get(csrf);
    console.log(params);
    const scan_details = await get_scan_details(csrfToken, params.project_id, params.scan_id);
    if (isError(scan_details)) {
        console.error(scan_details);
    }
    return scan_details;
};
