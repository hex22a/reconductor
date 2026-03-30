import { afterEach, describe, expect, mock, test } from 'bun:test';
import {
    createProjectResolver,
    type ProjectResolver,
    type ProjectResolverFactoryDeps,
} from './project.resovler';
import type { ProjectService } from './project.service';

describe('project.resolver', () => {
    const mockCreateProject = mock();
    const mockGetProject = mock();
    const mockListProjects = mock();
    const mockProjectService: ProjectService = {
        createProject: mockCreateProject,
        getProject: mockGetProject,
        listProjects: mockListProjects,
    };
    const expectedProjectResolverFactoryDeps: ProjectResolverFactoryDeps = {
        projectService: mockProjectService,
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
                project: mockGetProject,
                projects: mockListProjects,
            },
            Mutation: {
                createProject: mockCreateProject,
            },
        };
        // Act
        const actualProjectResolver: ProjectResolver = createProjectResolver(
            expectedProjectResolverFactoryDeps,
        );
        // Assert
        expect(actualProjectResolver).toEqual(expectedProjectResolver);
    });
});
