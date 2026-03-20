import type { GraphQlContext } from '@/src/transport/graphql.context';
import type { ProjectRepository } from '@/src/persistence/project.db';
import type { ProjectDto, ProjectsDto } from '@/src/transport/project.dto';
import type { ProjectEntity } from '@/src/domain/project.entity';
import { type CursorDecoder, type CursorEncoder } from '@/src/utils/cursor';

export type ProjectResolverArgs = {
    id: string;
};

export type CreateProjectArgs = {
    input: {
        name: string;
    };
};

export type ProjectResolverFactoryDeps = {
    projectRepository: ProjectRepository;
    encodeCursor: CursorEncoder;
    decodeCursor: CursorDecoder;
};

export type ProjectResolver = {
    Query: {
        project: (parent: unknown, args: ProjectResolverArgs) => Promise<ProjectDto>;
        projects: (parent: unknown, args: unknown, context: GraphQlContext) => Promise<ProjectsDto>;
    };
    Mutation: {
        createProject: (
            parent: unknown,
            args: CreateProjectArgs,
            context: GraphQlContext,
        ) => Promise<ProjectDto>;
    };
};

export function createProjectResolver({
    projectRepository,
    encodeCursor,
}: ProjectResolverFactoryDeps): ProjectResolver {
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
            async projects(_, __, context) {
                const { projects, hasNextPage } = await projectRepository.listProjects(
                    context.user.id,
                );
                const edges = projects.map((projectEnity) => ({
                    node: {
                        id: projectEnity.id,
                        name: projectEnity.name,
                        created_at: projectEnity.created_at,
                    },
                    cursor: encodeCursor(projectEnity.id),
                }));
                return {
                    edges,
                    pageInfo: {
                        hasNextPage,
                        hasPreviousPage: false,
                        startCursor: edges.at(0)?.cursor ?? null,
                        endCursor: edges.at(-1)?.cursor ?? null,
                    },
                };
            },
        },
        Mutation: {
            async createProject(
                _,
                { input: { name } }: CreateProjectArgs,
                context: GraphQlContext,
            ): Promise<ProjectDto> {
                const project: ProjectEntity = await projectRepository.createProject({
                    name,
                    owner_id: context.user.id,
                });
                return {
                    id: project.id,
                    name,
                    created_at: project.created_at,
                };
            },
        },
    };
}
