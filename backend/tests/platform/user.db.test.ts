import { describe, expect, it, test } from 'bun:test';
import { withTrx } from '../helpers';
import { createUserRepository, type UserRepository } from '@/src/persistence/user.db';

describe('user.db', () => {
    test('createUserRepository', async () => {
        await withTrx(async (trx) => {
            // Arrange
            const expectedUserRepository: UserRepository = {
                addUser: expect.any(Function),
            };
            // Act
            const actualUserRepository = createUserRepository(trx);
            // Assert
            expect(actualUserRepository).toEqual(expectedUserRepository);
        });
    });
    it('adds a user in database', () => {
        // Arrange
        // Act
        // Assert
    });
});
