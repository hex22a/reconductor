import { describe, expect, it, test } from 'bun:test';
import { catchRollback, withTrx } from '../decorators';
import { createUserRepository, type UserRepository } from '@/src/persistence/user.db';
import type { UserEntity } from '@/src/domain/user.entity';
import { createUserFixture, expectedExistingUserPasswordHash, expectedExistingUserUsername } from '../fixtures/users';
import { UserNotFoundError } from '@/src/domain/errors/UserNotFoundError';

describe('user.db', () => {
    test('createUserRepository', async () => {
        await catchRollback(async () => {
            await withTrx(async (trx) => {
                // Arrange
                const expectedUserRepository: UserRepository = {
                    addUser: expect.any(Function),
                    getUserByUsername: expect.any(Function),
                };
                // Act
                const actualUserRepository: UserRepository = createUserRepository(trx);
                // Assert
                expect(actualUserRepository).toEqual(expectedUserRepository);
            });
        });
    });

    describe('addUser', () => {
        it('adds a user in database', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedUsername = 'not_root';
                    const expectedPassowrdHash = 'another_hash';
                    const [expectedUserInsert, expectedAddedUser] = createUserFixture(expectedUsername, expectedPassowrdHash);
                    const userRepository: UserRepository = createUserRepository(trx);
                    // Act
                    const actualAddedUser: UserEntity = await userRepository.addUser(expectedUserInsert);
                    // Assert
                    expect(actualAddedUser.id).toBeString();
                    expect(actualAddedUser.created_at).toBeDate();
                    expect(actualAddedUser.updated_at).toBeDate();
                    expect(actualAddedUser.last_login_at).toBeDate();
                    expect(actualAddedUser.username).toEqual(expectedAddedUser.username);
                    expect(actualAddedUser.password_hash).toEqual(expectedAddedUser.password_hash);
                    expect(actualAddedUser.password_version).toEqual(expectedAddedUser.password_version);
                    expect(actualAddedUser.is_active).toEqual(expectedAddedUser.is_active);
                });
            });
        });

        it('throws an error when attemting to add user with existing username', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedErrorCode = '23505';
                    const [expectedUserInsert] = createUserFixture(expectedExistingUserUsername, expectedExistingUserPasswordHash);
                    const userRepository: UserRepository = createUserRepository(trx);
                    // Act
                    try {
                        await userRepository.addUser(expectedUserInsert);
                    } catch (actualError) {
                        // Assert
                        expect(actualError).toBeInstanceOf(Bun.SQL.PostgresError)
                        expect((actualError as Bun.SQL.PostgresError).errno).toEqual(expectedErrorCode);
                    }
                });
            });
        });
    });

    describe('getUserByUsername', () => {
        it('returns a user if exists', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const [, expectedUser] = createUserFixture(expectedExistingUserUsername, expectedExistingUserPasswordHash);
                    const userRepository: UserRepository = createUserRepository(trx);
                    // Act
                    const actualUser: UserEntity = await userRepository.getUserByUsername(expectedExistingUserUsername);
                    // Assert
                    expect(actualUser.id).toBeString();
                    expect(actualUser.created_at).toBeDate();
                    expect(actualUser.updated_at).toBeDate();
                    expect(actualUser.last_login_at).toBeDate();
                    expect(actualUser.username).toEqual(expectedUser.username);
                    expect(actualUser.password_hash).toEqual(expectedUser.password_hash);
                    expect(actualUser.password_version).toEqual(expectedUser.password_version);
                    expect(actualUser.is_active).toEqual(expectedUser.is_active);
                });
            });
        });

        it('throws an error if user is not found', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedUsername = 'not_root';
                    const userRepository: UserRepository = createUserRepository(trx);
                    // Act
                    try {
                        await userRepository.getUserByUsername(expectedUsername);
                    } catch (actualError) {
                        // Assert
                        expect(actualError).toBeInstanceOf(UserNotFoundError);
                    }
                });
            });
        });
    });
});
