import { ProjectNotFoundError } from "../domain/errors/ProjectNotFoundError";
import type { ProjectEntity, ProjectInsert } from "../domain/project.entity";
import type { SQL } from "./db";

export type ProjectRepositoryDeps = {
    sql: SQL,
};

export type ProjectRepository = {
    createProject: (project: ProjectInsert) => Promise<ProjectEntity>;
    getProject: (projectId: string) => Promise<ProjectEntity>;
};

export function createProjectRepository({ sql }: ProjectRepositoryDeps): ProjectRepository {
    return {
        async createProject(project: ProjectInsert): Promise<ProjectEntity> {
            const queryResults = await sql`
                INSERT INTO recon.projects
                    (name, owner_id)
                VALUES
                    (${project.name}, ${project.owner_id})
                RETURNING *;
            `;
            return queryResults[0];
        },
        async getProject(projectId: string): Promise<ProjectEntity> {
            const [project] = await sql<Array<ProjectEntity>>`
                SELECT
                    id,
                    owner_id,
                    name,
                    created_at
                FROM recon.projects
                WHERE id=${projectId}
                LIMIT 1;
            `;
            if (!project) {
                throw new ProjectNotFoundError();
            }
            return project;
        },
    };
};
