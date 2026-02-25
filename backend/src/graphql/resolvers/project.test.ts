import { afterEach, describe, expect, mock, test } from 'bun:test';
import {
    createProjectResolver,
    type ProjectResolver,
    type ProjectResolverFactoryDeps,
} from './project';
import type { ProjectRepository } from '../persistence/project.db';
import type { ProjectDto } from '../transport/project.dto';
import { createProjectFixture } from '@/tests/fixtures/projects';

describe('project', () => {
    const expectedParent = null;
    const mockCreateProject = mock();
    const mockGetProject = mock();
    const mockProjectRepository: ProjectRepository = {
        createProject: mockCreateProject,
        getProject: mockGetProject,
    };
    const expectedProjectResolverFactoryDeps: ProjectResolverFactoryDeps = {
        projectRepository: mockProjectRepository,
    };

    afterEach(() => {
        mockCreateProject.mockReset();
        mockGetProject.mockReset();
    });

    test('createProjectResolver', () => {
        // Arrange
        const expectedProjectResolver: ProjectResolver = {
            Query: {
                project: expect.any(Function),
                projects: expect.any(Function),
            },
        };
        // Act
        const actualProjectResolver: ProjectResolver = createProjectResolver(
            expectedProjectResolverFactoryDeps,
        );
        // Assert
        expect(actualProjectResolver).toEqual(expectedProjectResolver);
    });

    test('project resolver', async () => {
        // Arrange
        const expectedOwnerId = 'owner_id';
        const expectedProjectId = 'id';
        const expectedProjectName = 'name';
        const expectedProject: ProjectDto = {
            id: expectedProjectId,
            name: expectedProjectName,
            created_at: expect.any(Date),
        };
        const [expectedProjectEntity] = createProjectFixture(
            expectedProjectName,
            expectedOwnerId,
            expectedProjectId,
        );
        const expectedArgs = { id: expectedProjectId };
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
});
