import { projectsStore } from '@/lib/components/projects/projects.svelte';
import { isError } from '@/lib/transport/ErrorResponse';

export async function load({ parent }: { parent: () => Promise<void> }) {
    await parent();
    const res = await projectsStore.list();
    if (isError(res)) {
        console.error(res);
    }
    return res;
}
