import type { ProjectRepository } from '../persistence/project.db';
import type { ProjectDto } from '../transport/project.dto';

type ProjectResolverArgs = {
    id: string;
};

export type ProjectResolverFactoryDeps = {
    projectRepository: ProjectRepository;
};

export type ProjectResolver = {
    Query: {
        project: (parent: unknown, args: ProjectResolverArgs) => Promise<ProjectDto>;
        projects: () => Promise<void>;
    };
};

export function createProjectResolver({
    projectRepository,
}: ProjectResolverFacrotyDeps): ProjectResolver {
    return {
        Query: {
            async project(_, args) {
                const project = await projectRepository.getProject(args.id);
                return {
                    id: project.id,
                    name: project.name,
                    created_at: project.created_at,
                };
            },
            async projects() {
                throw new Error('not implemented');
            },
        },
    };
}
