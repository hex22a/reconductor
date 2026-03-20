import { describe, expect, it, test } from 'bun:test';
import { catchRollback, withTrx } from '../decorators';
import { createProjectRepository, type ProjectRepository } from '@/src/persistence/project.db';
import { createProjectFixture } from '../fixtures/projects';
import type { ProjectEntity } from '@/src/domain/project.entity';
import { ProjectNotFoundError } from '@/src/domain/errors/ProjectNotFoundError';
import { expectedExistingUserId } from '../fixtures/users';

describe('project.db', () => {
    test('createProjectRepository', async () => {
        await catchRollback(async () => {
            await withTrx(async (trx) => {
                // Arrange
                const expectedProjectRepository: ProjectRepository = {
                    createProject: expect.any(Function),
                    getProject: expect.any(Function),
                    listProjects: expect.any(Function),
                };
                // Act
                const actualUserRepository: ProjectRepository = createProjectRepository({
                    sql: trx,
                });
                // Assert
                expect(actualUserRepository).toEqual(expectedProjectRepository);
            });
        });
    });

    describe('createProject', () => {
        it('creates a project in database', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedProjectName = 'test';
                    const [, expecteProjectInsert] = createProjectFixture(
                        expectedProjectName,
                        expectedExistingUserId,
                    );
                    const actualProjectRepository: ProjectRepository = createProjectRepository({
                        sql: trx,
                    });
                    // Act
                    const actualCreatedProject: ProjectEntity =
                        await actualProjectRepository.createProject(expecteProjectInsert);
                    // Assert
                    expect(actualCreatedProject.id).toBeString();
                    expect(actualCreatedProject.name).toEqual(expectedProjectName);
                    expect(actualCreatedProject.owner_id).toEqual(expectedExistingUserId);
                    expect(actualCreatedProject.created_at).toBeDate();
                });
            });
        });

        it('throws an error when attemting to add a project with wrong owner id', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedErrorCode = '23503';
                    const expectedProjectName = 'test';
                    const expectedWrongOwnerId = '5ffe9624-5f76-4534-b804-a569613822d0';
                    const [, expectedProjectInsert] = createProjectFixture(
                        expectedProjectName,
                        expectedWrongOwnerId,
                    );
                    const projectRepository: ProjectRepository = createProjectRepository({
                        sql: trx,
                    });
                    // Act
                    try {
                        await projectRepository.createProject(expectedProjectInsert);
                    } catch (actualError) {
                        // Assert
                        expect(actualError).toBeInstanceOf(Bun.SQL.PostgresError);
                        expect((actualError as Bun.SQL.PostgresError).errno).toEqual(
                            expectedErrorCode,
                        );
                    }
                });
            });
        });
    });

    describe('getProject', () => {
        it('returns a project if exists', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedProjectName = 'test';
                    const [, expectedProjectInsert] = createProjectFixture(
                        expectedProjectName,
                        expectedExistingUserId,
                    );
                    const projectRepository: ProjectRepository = createProjectRepository({
                        sql: trx,
                    });
                    const expectedProject: ProjectEntity =
                        await projectRepository.createProject(expectedProjectInsert);
                    // Act
                    const actualProject: ProjectEntity = await projectRepository.getProject(
                        expectedProject.id,
                    );
                    // Assert
                    expect(actualProject.id).toEqual(expectedProject.id);
                    expect(actualProject.name).toEqual(expectedProjectName);
                    expect(actualProject.owner_id).toEqual(expectedExistingUserId);
                    expect(actualProject.created_at).toEqual(actualProject.created_at);
                });
            });
        });

        it('throws an error if project not found', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedProjectName = '57455bc2-af34-4116-a98f-7dce209eba35';
                    const projectRepository: ProjectRepository = createProjectRepository({
                        sql: trx,
                    });
                    // Act
                    try {
                        await projectRepository.getProject(expectedProjectName);
                    } catch (actualError) {
                        // Assert
                        expect(actualError).toBeInstanceOf(ProjectNotFoundError);
                    }
                });
            });
        });
    });

    describe('listProjects', () => {
        it('returns list of all projects for current user', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const projectRepository: ProjectRepository = createProjectRepository({
                        sql: trx,
                    });
                    // Act
                    const { projects, hasNextPage } =
                        await projectRepository.listProjects(expectedExistingUserId);
                    // Assert
                    expect(projects).toHaveLength(1);
                    expect(hasNextPage).toBeFalse();
                });
            });
        });

        it('returns list of all projects for current user after cursor', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedAnchorProjectName = 'anchor';
                    const expectedProject1Name = 'test';
                    const expectedProject2Name = 'test2';
                    const [, expectedAnchorProjectInsert] = createProjectFixture(
                        expectedAnchorProjectName,
                        expectedExistingUserId,
                    );
                    const [, expectedProject1Insert] = createProjectFixture(
                        expectedProject1Name,
                        expectedExistingUserId,
                    );
                    const [, expectedProject2Insert] = createProjectFixture(
                        expectedProject2Name,
                        expectedExistingUserId,
                    );
                    const projectRepository: ProjectRepository = createProjectRepository({
                        sql: trx,
                    });
                    await projectRepository.createProject(expectedProject1Insert);
                    await projectRepository.createProject(expectedProject2Insert);
                    const expectedAnchorProject = await projectRepository.createProject(
                        expectedAnchorProjectInsert,
                    );
                    // Act
                    const { projects, hasNextPage } = await projectRepository.listProjects(
                        expectedExistingUserId,
                        expectedAnchorProject.id,
                    );
                    // Assert
                    expect(projects).toHaveLength(2);
                    expect(hasNextPage).toBeFalse();
                });
            });
        });
    });
});
