import { afterEach, describe, expect, mock, test } from 'bun:test';
import {
    createProjectResolver,
    type CreateProjectArgs,
    type ProjectResolver,
    type ProjectResolverArgs,
    type ProjectResolverFactoryDeps,
} from './project';
import type { ProjectRepository } from '@/src/persistence/project.db';
import type { ProjectDto } from '@/src/transport/project.dto';
import { createProjectFixture } from '@/tests/fixtures/projects';
import type { GraphQlContext } from '@/src/transport/graphql.context';
import type { ProjectEntity } from '@/src/domain/project.entity';

describe('project', () => {
    const expectedParent = null;
    const mockCreateProject = mock();
    const mockGetProject = mock();
    const mockListProjects = mock();
    const mockProjectRepository: ProjectRepository = {
        createProject: mockCreateProject,
        getProject: mockGetProject,
        listProjects: mockListProjects,
    };
    const expectedProjectResolverFactoryDeps: ProjectResolverFactoryDeps = {
        projectRepository: mockProjectRepository,
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
    });

    test('createProjectResolver', () => {
        // Arrange
        const expectedProjectResolver: ProjectResolver = {
            Query: {
                project: expect.any(Function),
                projects: expect.any(Function),
            },
            Mutation: {
                createProject: expect.any(Function),
            },
        };
        // Act
        const actualProjectResolver: ProjectResolver = createProjectResolver(
            expectedProjectResolverFactoryDeps,
        );
        // Assert
        expect(actualProjectResolver).toEqual(expectedProjectResolver);
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
        const projectResolver: ProjectResolver = createProjectResolver(
            expectedProjectResolverFactoryDeps,
        );
        // Act
        const actualProject: ProjectDto = await projectResolver.Query.project(
            expectedParent,
            expectedArgs,
        );
        // Assert
        expect(actualProject).toEqual(expectedProject);
    });

    test('createProject', async () => {
        // Arrange
        const expectedUserId = '019c9abc-a10c-76e3-8287-885036664a5c';
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
        const projectResolver: ProjectResolver = createProjectResolver(
            expectedProjectResolverFactoryDeps,
        );
        // Act
        const actualProject: ProjectDto = await projectResolver.Mutation.createProject(
            expectedParent,
            expectedArgs,
            expectedContext,
        );
        // Assert
        expect(actualProject).toEqual(expectedProject);
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
        mockListProjects.mockResolvedValue(expectedProjectEntities);
        const expectedProjects: Array<ProjectDto> = [expectedProject];
        const projectResolver: ProjectResolver = createProjectResolver(
            expectedProjectResolverFactoryDeps,
        );
        // Act
        const actualProjects: Array<ProjectDto> = await projectResolver.Query.projects(
            expectedParent,
            expectedArgs,
            expectedContext,
        );
        // Assert
        expect(actualProjects).toEqual(expectedProjects);
        expect(mockListProjects).toHaveBeenCalledWith(expectedUserId);
    });
});
