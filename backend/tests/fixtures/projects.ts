import type { ProjectEntity, ProjectInsert, ProjectInsertSeed } from '@/src/domain/project.entity';
import { randomUUIDv7 } from 'bun';

export const expectedExistingProjectId = '7c04492e-ee12-4508-b59e-66c9102616d0';
export const expectedExistingProjectName = 'existing_project';

export function createProjectFixture(
    name: string,
    ownerId: string,
    projectId?: string,
): [ProjectEntity, ProjectInsert, ProjectInsertSeed] {
    const id = projectId || randomUUIDv7();
    const date = new Date();
    return [
        { id, name, owner_id: ownerId, created_at: date },
        { name, owner_id: ownerId },
        { id, name, owner_id: ownerId },
    ];
}
