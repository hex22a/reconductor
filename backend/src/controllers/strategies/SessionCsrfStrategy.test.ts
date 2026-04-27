import { describe, expect, mock, test } from 'bun:test';
import type { CsrfProvider } from '@/src/providers/csrf';
import { SessionCsrfStrategy, type SessionCsrfStrategyDeps } from './SessionCsrfStrategy';
import type { BunRequest } from 'bun';
import type { RequestContext } from '../types';
import type { UserSession } from '@/src/domain/session.entity';
import { CSRF_HEADER } from '@/src/constants';

describe('SessionCsrfStrategy', () => {
    const expectedCsrfToken = 'csrf_token';
    const expectedSessionToken = 'random_token';
    const expectedUsername = 'test';
    const expectedUserId = '019dcef9-9208-7748-ac23-ed2e7467e806';
    const mockGenerateCsrf = mock();
    const mockVerifyCsrf = mock();
    const mockGetHeader = mock();

    const mockCsrfProvider: CsrfProvider = {
        generate: mockGenerateCsrf,
        verify: mockVerifyCsrf,
    };
    const mockAnonymousCsrfStrategyDeps: SessionCsrfStrategyDeps = {
        csrfProvider: mockCsrfProvider,
    };
    const mockHeaders = {
        get: mockGetHeader,
    } satisfies Partial<Headers>;
    const expectedRequest = {
        headers: mockHeaders as unknown as Headers,
    } satisfies Partial<BunRequest>;

    test('constructor', () => {
        // Arrange
        // Act
        const actualSessionCsrfStrategy = new SessionCsrfStrategy(mockAnonymousCsrfStrategyDeps);
        // Assert
        expect(actualSessionCsrfStrategy.verifyCsrfToken).toBeFunction();
        expect(actualSessionCsrfStrategy.csrfProvider).toEqual(mockCsrfProvider);
    });

    describe('verifyCsrfToken', () => {
        test('return true if csrf is valid and found in repository', async () => {
            // Arrange
            const expectedUserSession: UserSession = {
                csrfToken: expectedCsrfToken,
                token: expectedSessionToken,
                userId: expectedUserId,
                username: expectedUsername,
            };
            const expectedContext: RequestContext = {
                user: expectedUserSession,
            };
            mockVerifyCsrf.mockReturnValue(true);
            mockGetHeader.mockReturnValue(expectedCsrfToken);
            const sessionCsrfStrategy = new SessionCsrfStrategy(mockAnonymousCsrfStrategyDeps);
            // Act
            const actualIsValid = await sessionCsrfStrategy.verifyCsrfToken(
                expectedRequest as unknown as BunRequest,
                expectedContext,
            );
            // Assert
            expect(actualIsValid).toBeTrue();
            expect(mockGetHeader).toHaveBeenCalledWith(CSRF_HEADER);
            expect(mockVerifyCsrf).toHaveBeenCalledWith(expectedCsrfToken);
        });

        test('return false if csrf is invalid', async () => {
            // Arrange
            const expectedUserSession: UserSession = {
                csrfToken: expectedCsrfToken,
                token: expectedSessionToken,
                userId: expectedUserId,
                username: expectedUsername,
            };
            const expectedContext: RequestContext = {
                user: expectedUserSession,
            };
            mockVerifyCsrf.mockReturnValue(false);
            mockGetHeader.mockReturnValue(expectedCsrfToken);
            const sessionCsrfStrategy = new SessionCsrfStrategy(mockAnonymousCsrfStrategyDeps);
            // Act
            const actualIsValid = await sessionCsrfStrategy.verifyCsrfToken(
                expectedRequest as unknown as BunRequest,
                expectedContext,
            );
            // Assert
            expect(actualIsValid).toBeFalse();
            expect(mockGetHeader).toHaveBeenCalledWith(CSRF_HEADER);
            expect(mockVerifyCsrf).toHaveBeenCalledWith(expectedCsrfToken);
        });

        test('return false if csrf is valid but not valid in repository', async () => {
            // Arrange
            const expectedInvalidCsrfToken = 'invalid_csrf';
            const expectedUserSession: UserSession = {
                csrfToken: expectedInvalidCsrfToken,
                token: expectedSessionToken,
                userId: expectedUserId,
                username: expectedUsername,
            };
            const expectedContext: RequestContext = {
                user: expectedUserSession,
            };
            mockVerifyCsrf.mockReturnValue(true);
            mockGetHeader.mockReturnValue(expectedCsrfToken);
            const sessionCsrfStrategy = new SessionCsrfStrategy(mockAnonymousCsrfStrategyDeps);
            // Act
            const actualIsValid = await sessionCsrfStrategy.verifyCsrfToken(
                expectedRequest as unknown as BunRequest,
                expectedContext,
            );
            // Assert
            expect(actualIsValid).toBeFalse();
            expect(mockGetHeader).toHaveBeenCalledWith(CSRF_HEADER);
            expect(mockVerifyCsrf).toHaveBeenCalledWith(expectedCsrfToken);
        });

        test('return false if csrf header not found', async () => {
            // Arrange
            const expectedInvalidCsrfToken = 'invalid_csrf';
            const expectedUserSession: UserSession = {
                csrfToken: expectedInvalidCsrfToken,
                token: expectedSessionToken,
                userId: expectedUserId,
                username: expectedUsername,
            };
            const expectedContext: RequestContext = {
                user: expectedUserSession,
            };
            mockVerifyCsrf.mockReturnValue(true);
            mockGetHeader.mockReturnValue(null);
            const sessionCsrfStrategy = new SessionCsrfStrategy(mockAnonymousCsrfStrategyDeps);
            // Act
            const actualIsValid = await sessionCsrfStrategy.verifyCsrfToken(
                expectedRequest as unknown as BunRequest,
                expectedContext,
            );
            // Assert
            expect(actualIsValid).toBeFalse();
            expect(mockGetHeader).toHaveBeenCalledWith(CSRF_HEADER);
            expect(mockVerifyCsrf).toHaveBeenCalledWith(expectedCsrfToken);
        });
    });
});
