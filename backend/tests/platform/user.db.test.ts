import { describe, expect, it, test } from 'bun:test';
import { catchRollback, withTrx } from '../decorators';
import { createUserRepository, type UserRepository } from '@/src/persistence/user.db';
import type { UserEntity } from '@/src/domain/user.entity';
import { createUserFixture } from '../fixtures/users';

describe('user.db', () => {
    test('createUserRepository', async () => {
        await catchRollback(async () => {
            await withTrx(async (trx) => {
                // Arrange
                const expectedUserRepository: UserRepository = {
                    addUser: expect.any(Function),
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
                    const userRepositoy: UserRepository = createUserRepository(trx);
                    // Act
                    const actualAddedUser: UserEntity = await userRepositoy.addUser(expectedUserInsert);
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
                    const expectedUsername = 'root';
                    const expectedPassowrdHash = 'another_hash';
                    const [expectedUserInsert] = createUserFixture(expectedUsername, expectedPassowrdHash);
                    const userRepositoy: UserRepository = createUserRepository(trx);
                    // Act
                    try {
                        await userRepositoy.addUser(expectedUserInsert);
                    } catch (actualError) {
                        // Assert
                        expect(actualError).toBeInstanceOf(Bun.SQL.PostgresError)
                        expect((actualError as Bun.SQL.PostgresError).errno).toEqual(expectedErrorCode);
                    }
                });
            });
        });
    });
});
