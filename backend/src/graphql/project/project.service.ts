import type { ProjectRepository } from '@/src/persistence/project.db';
import type { ProjectDto } from '@/src/transport/project.dto';
import type { CursorDecoder, CursorEncoder } from '@/src/utils/cursor';
import type { EntityResolver, MutationResolver, PaginatonResolver } from '../types';
import type { Pagination } from '@/src/transport/pagination.dto';
import type { Edge } from '@/src/transport/edge.dto';
import type { GraphQlContext } from '@/src/transport/graphql.context';
import type { CreateEntityPayload } from '@/src/transport/payload.dto';
import type { ProjectEntity } from '@/src/domain/project.entity';

export type ProjectResolverArgs = {
    id: string;
};

export type CreateProjectArgs = {
    input: {
        name: string;
    };
};
export type ProjectServiceFactoryDeps = {
    projectRepository: ProjectRepository;
    encodeCursor: CursorEncoder;
    decodeCursor: CursorDecoder;
};

export type ProjectService = {
    getProject: EntityResolver<ProjectDto, ProjectResolverArgs>;
    listProjects: PaginatonResolver<ProjectDto>;
    createProject: MutationResolver<ProjectDto, CreateProjectArgs>;
};

export function createProjectService({
    projectRepository,
    encodeCursor,
}: ProjectServiceFactoryDeps): ProjectService {
    return {
        async getProject(_, args) {
            const project = await projectRepository.getProject(args.id);
            return {
                id: project.id,
                name: project.name,
                created_at: project.created_at,
            };
        },
        async listProjects(_, __, context): Promise<Pagination<Edge<ProjectDto>>> {
            const { projects, hasNextPage } = await projectRepository.listProjects(context.user.id);
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
        async createProject(
            _,
            { input: { name } }: CreateProjectArgs,
            context: GraphQlContext,
        ): Promise<CreateEntityPayload<Edge<ProjectDto>>> {
            const project: ProjectEntity = await projectRepository.createProject({
                name,
                owner_id: context.user.id,
            });
            return {
                edge: {
                    node: {
                        id: project.id,
                        name,
                        created_at: project.created_at,
                    },
                    cursor: encodeCursor(project.id),
                },
                errors: [],
            };
        },
    };
}
