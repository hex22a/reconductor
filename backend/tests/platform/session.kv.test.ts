import { SessionNotFoundError } from '@/src/domain/errors/SessionNotFoundError';
import type { UserSession } from '@/src/domain/session.entity';
import { kv } from '@/src/persistence/kv';
import { createSessionRepository, type SessionRepository } from '@/src/persistence/session.kv';
import { afterEach, describe, expect, it, test } from 'bun:test';
import { createUserSessionFixture } from '../fixtures/sessions';

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
        const expectedUsername = 'user';
        const expectedUserId = '019c6c94-0fb1-7241-922f-b3eb297a5a2f';
        const expectedToken = 'random_value';
        const [expectedUserSession, expectedUserSessionInsert] = createUserSessionFixture(expectedToken, expectedUserId, expectedUsername);
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

    it('gets userSession from session storage', async () => {
        // Arrange
        const expectedUsername = 'user';
        const expectedToken = 'some_token';
        const expectedUserId = '019c6c94-0fb1-7241-922f-b3eb297a5a2f';
        const [expectedUserSession, expectedUserSessionInsert] = createUserSessionFixture(expectedToken, expectedUserId, expectedUsername);
        const sessionRepository = createSessionRepository(kv);
        await sessionRepository.createUserSession(expectedUserSessionInsert);
        // Act
        const actualUserSession: UserSession = await sessionRepository.getUserSession(expectedToken);
        // Assert
        expect(actualUserSession).toEqual(expectedUserSession);
    });
});
