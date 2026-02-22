import type { ProjectEntity, ProjectInsert } from "@/src/domain/project.entity";
import { randomUUIDv7 } from "bun";

export function createProjectFixture(
    name: string,
    ownerId: string,
): [ProjectInsert, ProjectEntity] {
    const id = randomUUIDv7();
    const date = new Date();
    return [
        { name, owner_id: ownerId },
        { id, name, owner_id: ownerId, created_at: date }
    ]
}
