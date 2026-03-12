import { describe, test, it, mock, afterEach, expect } from 'bun:test';
import type { SessionRepository } from '@/src/persistence/session.kv';
import { createLogoutController, type LogoutControllerDeps } from './logout';
import type { RequestContext, RequestHandler } from '../types';
import type { BunRequest } from 'bun';
import type { UserSession } from '@/src/domain/session.entity';
import { constants } from 'node:http2';

describe('logout controller', () => {
    const mockCreateUserSession = mock();
    const mockDeleteUserSession = mock();
    const mockGetUserSession = mock();
    const mockSessionRepository: SessionRepository = {
        createUserSession: mockCreateUserSession,
        getUserSession: mockGetUserSession,
        deleteUserSession: mockDeleteUserSession,
    };
    const expectedLogoutControllerDeps: LogoutControllerDeps = {
        sessionRepository: mockSessionRepository,
    };

    afterEach(() => {
        mockCreateUserSession.mockReset();
        mockGetUserSession.mockReset();
        mockDeleteUserSession.mockReset();
    });

    test('createLogoutController', () => {
        // Arrange
        // Act
        const actualLogout: RequestHandler<RequestContext> = createLogoutController(
            expectedLogoutControllerDeps,
        );
        // Assert
        expect(actualLogout).toBeFunction();
    });

    describe('logout controller', () => {
        it('returns 204 if session is deleted', async () => {
            // Arrange
            const expectedToken = 'token';
            const expectedUserId = '019cb419-2d24-727d-b66a-cf6390891464';
            const expectedUsername = 'username';
            const expectedUserSession: UserSession = {
                token: expectedToken,
                userId: expectedUserId,
                username: expectedUsername,
            };
            const expectedContext: RequestContext = {
                user: expectedUserSession,
            };
            const expectedRequest = {} satisfies Partial<BunRequest>;
            const expectedResponse: Response = new Response(null, {
                status: constants.HTTP_STATUS_NO_CONTENT,
            });
            const logout: RequestHandler<RequestContext> = createLogoutController(
                expectedLogoutControllerDeps,
            );
            // Act
            const actualResponse: Response = await logout(
                expectedRequest as unknown as BunRequest,
                expectedContext,
            );
            // Assert
            expect(mockDeleteUserSession).toHaveBeenLastCalledWith(expectedToken);
            expect(actualResponse.status).toEqual(expectedResponse.status);
        });
    });
});
