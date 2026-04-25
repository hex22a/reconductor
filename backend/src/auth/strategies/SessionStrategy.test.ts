import { describe, test, it, mock, afterEach, expect } from 'bun:test';
import type { BunRequest, CookieMap } from 'bun';
import type { SessionRepository } from '@/src/persistence/session.kv';
import { USER_SESSION_COOKIE_NAME } from '@/src/constants';
import type { UserSession } from '@/src/domain/session.entity';
import { SessionNotFoundError } from '@/src/domain/errors/SessionNotFoundError';
import { SessionStrategy, type AuthStrategyDeps } from './SessionStrategy';
import { UnauthorizedError } from '@/src/domain/errors/UnauthorizedError';

describe('auth', () => {
    const mockCreateUserSession = mock();
    const mockGetUserSession = mock();
    const mockGetCookies = mock();
    const mockController = mock();
    const mockDeleteUserSession = mock();
    const mockSessionRepository: SessionRepository = {
        createUserSession: mockCreateUserSession,
        getUserSession: mockGetUserSession,
        deleteUserSession: mockDeleteUserSession,
    };
    const mockCookies = {
        get: mockGetCookies,
    } satisfies Partial<CookieMap>;
    const expectedRequest = {
        cookies: mockCookies as unknown as CookieMap,
    } satisfies Partial<BunRequest>;
    const expectedAuthStrategyDeps: AuthStrategyDeps = {
        sessionRepository: mockSessionRepository,
    };

    afterEach(() => {
        mockController.mockReset();
        mockCreateUserSession.mockReset();
        mockGetUserSession.mockReset();
        mockDeleteUserSession.mockReset();
    });

    test('constructor', () => {
        // Arrange
        // Act
        const actualStrategy: SessionStrategy = new SessionStrategy(expectedAuthStrategyDeps);
        // Assert
        expect(actualStrategy.sessionRepository).toEqual(mockSessionRepository);
        expect(actualStrategy.authenticate).toBeFunction();
    });

    describe('authenticate', () => {
        it('returns user session if session if found', async () => {
            // Arrange
            const expectedUserId = '019c94f3-4826-7c85-a663-98055fe5cba4';
            const expectedUsername = 'username';
            const expectedToken = 'token';
            const expectedCsrfToken = 'csrf_token';
            const expectedUserSession: UserSession = {
                userId: expectedUserId,
                username: expectedUsername,
                csrfToken: expectedCsrfToken,
                token: expectedToken,
            };
            mockGetCookies.mockReturnValue(expectedToken);
            mockGetUserSession.mockReturnValue(expectedUserSession);

            const strategy: SessionStrategy = new SessionStrategy(expectedAuthStrategyDeps);
            // Act
            const actualUserSession: UserSession = await strategy.authenticate(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(actualUserSession).toEqual(expectedUserSession);
            expect(mockGetCookies).toHaveBeenLastCalledWith(USER_SESSION_COOKIE_NAME);
            expect(mockGetUserSession).toHaveBeenLastCalledWith(expectedToken);
        });

        it('thorws UnauthorizedError if cookie not found', async () => {
            // Arrange
            mockGetCookies.mockReturnValue(null);

            const strategy: SessionStrategy = new SessionStrategy(expectedAuthStrategyDeps);
            try {
                // Act
                await strategy.authenticate(expectedRequest as unknown as BunRequest);
            } catch (actualError) {
                // Assert
                expect(actualError).toBeInstanceOf(UnauthorizedError);
                expect(mockGetCookies).toHaveBeenLastCalledWith(USER_SESSION_COOKIE_NAME);
            }
        });

        it('throws UnauthorizedError if session not found', async () => {
            // Arrange
            const expectedToken = 'token';
            mockGetCookies.mockReturnValue(expectedToken);
            mockGetUserSession.mockImplementation(() => {
                throw new SessionNotFoundError();
            });

            const strategy: SessionStrategy = new SessionStrategy(expectedAuthStrategyDeps);
            try {
                // Act
                await strategy.authenticate(expectedRequest as unknown as BunRequest);
            } catch (actualError) {
                // Assert
                expect(actualError).toBeInstanceOf(UnauthorizedError);
                expect(mockGetCookies).toHaveBeenLastCalledWith(USER_SESSION_COOKIE_NAME);
                expect(mockGetUserSession).toHaveBeenLastCalledWith(expectedToken);
            }
        });
    });
});
