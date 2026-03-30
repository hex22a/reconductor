import { afterEach, describe, expect, mock, test } from 'bun:test';
import type { ProjectRepository } from '@/src/persistence/project.db';
import type { ProjectDto } from '@/src/transport/project.dto';
import {
    createProjectService,
    type CreateProjectArgs,
    type ProjectResolverArgs,
    type ProjectService,
    type ProjectServiceFactoryDeps,
} from './project.service';
import type { ValidationError } from '@/src/transport/error.dto';
import { createProjectFixture } from '@/tests/fixtures/projects';
import type { GraphQlContext } from '@/src/transport/graphql.context';
import type { CreateEntityPayload } from '@/src/transport/payload.dto';
import type { Edge } from '@/src/transport/edge.dto';
import type { ProjectEntity } from '@/src/domain/project.entity';
import type { PageInfo } from '@/src/transport/pageInfo.dto';
import type { Pagination } from '@/src/transport/pagination.dto';

describe('project.service', () => {
    const expectedParent = null;
    const mockCreateProject = mock();
    const mockGetProject = mock();
    const mockListProjects = mock();
    const mockEncodeCursor = mock();
    const mockDecodeCursor = mock();
    const mockProjectRepository: ProjectRepository = {
        createProject: mockCreateProject,
        getProject: mockGetProject,
        listProjects: mockListProjects,
    };
    const expectedProjectServiceFactoryDeps: ProjectServiceFactoryDeps = {
        projectRepository: mockProjectRepository,
        encodeCursor: mockEncodeCursor,
        decodeCursor: mockDecodeCursor,
    };

    const expectedOwnerId = 'owner_id';
    const expectedProjectId = 'id';
    const expectedProjectName = 'name';
    const expectedProject: ProjectDto = {
        id: expectedProjectId,
        name: expectedProjectName,
        created_at: expect.any(Date),
    };

    afterEach(() => {
        mockCreateProject.mockReset();
        mockGetProject.mockReset();
        mockListProjects.mockReset();
        mockEncodeCursor.mockReset();
        mockDecodeCursor.mockReset();
    });

    test('createProjectService', () => {
        // Arrange
        const expectedProjectService: ProjectService = {
            getProject: expect.any(Function),
            listProjects: expect.any(Function),
            createProject: expect.any(Function),
        };
        // Act
        const actualProjectService: ProjectService = createProjectService(
            expectedProjectServiceFactoryDeps,
        );
        // Assert
        expect(actualProjectService).toEqual(expectedProjectService);
    });

    test('project', async () => {
        // Arrange
        const [expectedProjectEntity] = createProjectFixture(
            expectedProjectName,
            expectedOwnerId,
            expectedProjectId,
        );
        const expectedArgs: ProjectResolverArgs = { id: expectedProjectId };
        mockGetProject.mockResolvedValue(expectedProjectEntity);
        const projectService: ProjectService = createProjectService(
            expectedProjectServiceFactoryDeps,
        );
        // Act
        const actualProject: ProjectDto = await projectService.getProject(
            expectedParent,
            expectedArgs,
        );
        // Assert
        expect(actualProject).toEqual(expectedProject);
    });

    test('createProject', async () => {
        // Arrange
        const expectedUserId = '019c9abc-a10c-76e3-8287-885036664a5c';
        const expectedCursor = 'cursor';
        const expectedValidationErrors: Array<ValidationError> = [];
        const expectedArgs: CreateProjectArgs = {
            input: {
                name: expectedProjectName,
            },
        };
        const expectedContext: GraphQlContext = {
            user: { id: expectedUserId },
        };
        const [expectedProjectEntity, expectedProjectInsert] = createProjectFixture(
            expectedProjectName,
            expectedUserId,
            expectedProjectId,
        );
        mockCreateProject.mockResolvedValue(expectedProjectEntity);
        mockEncodeCursor.mockReturnValue(expectedCursor);
        const projectService: ProjectService = createProjectService(
            expectedProjectServiceFactoryDeps,
        );
        // Act
        const actualCreateProjectPayload: CreateEntityPayload<Edge<ProjectDto>> =
            await projectService.createProject(expectedParent, expectedArgs, expectedContext);
        // Assert
        expect(actualCreateProjectPayload.edge!.node).toEqual(expectedProject);
        expect(actualCreateProjectPayload.edge!.cursor).toEqual(expectedCursor);
        expect(actualCreateProjectPayload.errors).toEqual(expectedValidationErrors);
        expect(mockCreateProject).toHaveBeenCalledWith(expectedProjectInsert);
    });

    test('projects', async () => {
        // Arrange
        const expectedUserId = '019c9abc-a10c-76e3-8287-885036664a5c';
        const expectedArgs = null;
        const expectedContext: GraphQlContext = {
            user: { id: expectedUserId },
        };
        const [expectedProjectEntity] = createProjectFixture(
            expectedProjectName,
            expectedUserId,
            expectedProjectId,
        );
        const expectedProjectEntities: Array<ProjectEntity> = [expectedProjectEntity];
        const expectedHasNextPage = true;
        const expectedCursor = 'cursor';
        const expectedPageInfo: PageInfo = {
            hasNextPage: expectedHasNextPage,
            hasPreviousPage: false,
            startCursor: expectedCursor,
            endCursor: expectedCursor,
        };
        mockListProjects.mockResolvedValue({
            projects: expectedProjectEntities,
            hasNextPage: expectedHasNextPage,
        });
        mockEncodeCursor.mockReturnValue(expectedCursor);
        const expectedEdges: Array<Edge<ProjectDto>> = [
            {
                node: expectedProject,
                cursor: expectedCursor,
            },
        ];
        const projectService: ProjectService = createProjectService(
            expectedProjectServiceFactoryDeps,
        );
        // Act
        const actualProjects: Pagination<Edge<ProjectDto>> = await projectService.listProjects(
            expectedParent,
            expectedArgs,
            expectedContext,
        );
        // Assert
        expect(actualProjects.edges).toEqual(expectedEdges);
        expect(actualProjects.pageInfo).toEqual(expectedPageInfo);
        expect(mockListProjects).toHaveBeenCalledWith(expectedUserId);
    });
});
