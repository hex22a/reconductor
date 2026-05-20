import { create, list } from '@/lib/services/projects';
import { csrf } from '@/lib/stores/csrf';
import { isError, type ErrorResponse } from '@/lib/transport/ErrorResponse';
import type { Project } from '@/lib/transport/Project';
import { get } from 'svelte/store';

let projects = $state<Array<Project>>([]);

export const projectsStore = {
    get projects() {
        return projects;
    },
    async add(name: string): Promise<void | ErrorResponse> {
        const csrfToken = get(csrf);
        const response = await create(csrfToken, name);
        if (isError(response)) {
            return response;
        }
        projects.push(response);
    },
    async list(after?: string): Promise<void | ErrorResponse> {
        const csrfToken = get(csrf);
        const response = await list(csrfToken, after);
        if (isError(response)) {
            return response;
        }
        projects = projects.concat(response.data);
    },
};
