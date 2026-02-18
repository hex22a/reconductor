import { SessionNotFoundError } from '@/src/domain/errors/SessionNotFoundError';
import type { UserSession, UserSessionInsert } from '@/src/domain/session.entity';
import { kv } from '@/src/persistence/kv';
import { createSessionRepository, type SessionRepository } from '@/src/persistence/session.kv';
import { afterEach, describe, expect, it, test } from 'bun:test';

describe('session', () => {
    afterEach(async () => {
        await kv.send('FLUSHDB', ['ASYNC']);
    });

    test('createSessionRepository', () => {
        // Arrange
        const expectedSessionRepository: SessionRepository = {
            createUserSession: expect.any(Function),
            getUserSession: expect.any(Function),
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

    it('throws SessionNotFoundError if session is not found', async () => {
        // Arrange
        const expectedToken = 'does_not_exist';
        const sessionRepository = createSessionRepository(kv);
        // Act
        try {
            await sessionRepository.getUserSession(expectedToken);
        } catch (actualError) {
            // Assert
            expect(actualError).toBeInstanceOf(SessionNotFoundError);
        }
    });

    it('gets userId from session storage', async () => {
        // Arrange
        const expectedToken = 'some_token';
        const expectedUserId = '019c6c94-0fb1-7241-922f-b3eb297a5a2f';
        const expectedUserSession: UserSession = {
            userId: expectedUserId,
        };
        const expectedUserSessionInsert: UserSessionInsert = {
            token: expectedToken,
            userId: expectedUserId,
        };
        const sessionRepository = createSessionRepository(kv);
        await sessionRepository.createUserSession(expectedUserSessionInsert);
        // Act
        const actualUserSession: UserSession = await sessionRepository.getUserSession(expectedToken);
        // Assert
        expect(actualUserSession).toEqual(expectedUserSession);
    });
});
