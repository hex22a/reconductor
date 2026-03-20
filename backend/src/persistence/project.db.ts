import { PROJECTS_PAGE_SIZE } from '../constants';
import { ProjectNotFoundError } from '../domain/errors/ProjectNotFoundError';
import type { ProjectEntity, ProjectInsert } from '../domain/project.entity';
import type { SQL } from './db';

export type ProjectRepositoryDeps = {
    sql: SQL;
};

export interface ProjectRepository {
    createProject(project: ProjectInsert): Promise<ProjectEntity>;
    getProject(projectId: string): Promise<ProjectEntity>;
    listProjects(
        ownerId: string,
    ): Promise<{ projects: Array<ProjectEntity>; hasNextPage: boolean }>;
    listProjects(
        ownerId: string,
        cursorId: string,
    ): Promise<{ projects: Array<ProjectEntity>; hasNextPage: boolean }>;
}

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
        async listProjects(
            ownerId: string,
            cursorId?: string,
        ): Promise<{ projects: Array<ProjectEntity>; hasNextPage: boolean }> {
            const limit = PROJECTS_PAGE_SIZE + 1;
            const projects: Array<ProjectEntity> = await sql<Array<ProjectEntity>>`
                SELECT
                    id,
                    owner_id,
                    name,
                    created_at
                FROM recon.projects
                WHERE owner_id=${ownerId}
                ${cursorId ? sql`AND id < ${cursorId}` : sql``}
                ORDER BY id DESC
                LIMIT ${limit};
            `;
            const hasNextPage = projects.length === limit;
            return {
                projects: hasNextPage ? projects.slice(0, -1) : projects,
                hasNextPage,
            };
        },
    };
}
