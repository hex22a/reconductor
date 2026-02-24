import type { ProjectRepository } from "../persistence/project.db";
import type { ProjectDto } from "../transport/project.dto";

type ProjectResolverArgs = {
    id: string
};

export type ProjectResolver = {
    Query: {
        project: (
            parent: unknown,
            args: ProjectResolverArgs,
        ) => Promise<ProjectDto>;
        projects: () => Promise<void>;
    }
};

export function createProjectResolver(projectRepository: ProjectRepository): ProjectResolver {
    return {
        Query: {
            async project(_, args) {
                const project = await projectRepository.getProject(args.id);
                return {
                    id: project.id,
                    name: project.name,
                    createdAt: project.created_at,
                };
            },
            async projects() {
                throw new Error('not implemented');
            }
        }
    };
};
