import type { ProjectDto } from '@/src/transport/project.dto';
import type { EntityResolver, MutationResolver, PaginatonResolver } from '../types';
import type { CreateProjectArgs, ProjectResolverArgs, ProjectService } from './project.service';

export type ProjectResolverFactoryDeps = {
    projectService: ProjectService;
};

export type ProjectResolver = {
    Query: {
        project: EntityResolver<ProjectDto, ProjectResolverArgs>;
        projects: PaginatonResolver<ProjectDto>;
    };
    Mutation: {
        createProject: MutationResolver<ProjectDto, CreateProjectArgs>;
    };
};

export function createProjectResolver({
    projectService,
}: ProjectResolverFactoryDeps): ProjectResolver {
    return {
        Query: {
            project: projectService.getProject,
            projects: projectService.listProjects,
        },
        Mutation: {
            createProject: projectService.createProject,
        },
    };
}
