import type { UserSession, UserSessionInsert } from '@/src/domain/session.entity';
import { kv } from '@/src/persistence/kv';
import { createSessionRepository, type SessionRepository } from '@/src/persistence/session.kv';
import { describe, expect, it, test } from 'bun:test';

describe('session', () => {
    test('createSessionRepository', () => {
        // Arrange
        const expectedSessionRepository: SessionRepository = {
            createUserSession: expect.any(Function),
        };
        // Act
        const actualSessionRepository = createSessionRepository(kv);
        // Assert
        expect(actualSessionRepository).toEqual(expectedSessionRepository);
    });

    it('stores the sessions under user_session:{token} key', async () => {
        // Arrange
        const expectedUserId = '019c6c94-0fb1-7241-922f-b3eb297a5a2f';
        const expectedToken = 'random_value';
        const expectedUserSession: UserSession = {
            userId: expectedUserId,
        };
        const expectedUserSessionInsert: UserSessionInsert = {
            token: expectedToken,
            userId: expectedUserId,
        };
        const sessionRepository = createSessionRepository(kv);
        // Act
        const actualUserSession: UserSession = await sessionRepository.createUserSession(expectedUserSessionInsert);
        // Assert
        expect(actualUserSession).toEqual(expectedUserSession);
    });
});
