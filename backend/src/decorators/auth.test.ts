import { describe, test, it, mock, afterEach, expect } from 'bun:test';
import { constants } from 'node:http2';
import type { BunRequest, CookieMap } from 'bun';
import type { SessionRepository } from '../persistence/session.kv';
import { createAuthDecorators, type AuthDecorators, type AuthDecoratorsFactoryDeps } from './auth';
import { CORS_HEADERS, UNAUTHORIZED_ERROR_MESSAGE, USER_SESSION_COOKIE_NAME } from '../constants';
import type { UserSession } from '../domain/session.entity';
import { SessionNotFoundError } from '../domain/errors/SessionNotFoundError';

describe('auth', () => {
    const mockCreateUserSession = mock();
    const mockGetUserSession = mock();
    const mockGetCookies = mock();
    const mockController = mock();
    const mockSessionRepository: SessionRepository = {
        createUserSession: mockCreateUserSession,
        getUserSession: mockGetUserSession,
    };
    const mockCookies = {
        get: mockGetCookies,
    } satisfies Partial<CookieMap>;
    const expectedRequest = {
        cookies: mockCookies as unknown as CookieMap,
    } satisfies Partial<BunRequest>;
    const expectedAuthDecoratorsDeps: AuthDecoratorsFactoryDeps = {
        sessionRepository: mockSessionRepository,
    };

    afterEach(() => {
        mockController.mockReset();
        mockCreateUserSession.mockReset();
        mockGetUserSession.mockReset();
    });

    test('createAuthDecorators', () => {
        // Arrange
        const expectedAuthDecorators: AuthDecorators = {
            withAuth: expect.any(Function),
        };
        // Act
        const actualAuthDecorators = createAuthDecorators(expectedAuthDecoratorsDeps);
        // Assert
        expect(actualAuthDecorators).toEqual(expectedAuthDecorators);
    });

    describe('withAuth', () => {
        it('calls the controller if session is found', async () => {
            // Arrange
            const expectedUserId = '019c94f3-4826-7c85-a663-98055fe5cba4';
            const expectedUsername = 'username';
            const expectedToken = 'token';
            const expectedUserSession: UserSession = {
                userId: expectedUserId,
                username: expectedUsername,
            };
            const expectedResponseJson = { respose: 'some valid response' };
            const expectedResponseInit: ResponseInit = {
                headers: CORS_HEADERS,
            };
            const expectedResponse: Response = Response.json(
                expectedResponseJson,
                expectedResponseInit,
            );
            mockController.mockResolvedValue(expectedResponse);
            mockGetCookies.mockReturnValue(expectedToken);
            mockGetUserSession.mockReturnValue(expectedUserSession);

            const authDecorators = createAuthDecorators(expectedAuthDecoratorsDeps);
            const decoratedController = authDecorators.withAuth(mockController);
            // Act
            const actualResponse: Response = await decoratedController(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(mockGetCookies).toHaveBeenLastCalledWith(USER_SESSION_COOKIE_NAME);
            expect(mockGetUserSession).toHaveBeenLastCalledWith(expectedToken);
            expect(mockController).toHaveBeenCalledWith(expectedRequest);
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
            expect(actualResponse.status).toEqual(expectedResponse.status);
        });

        it('returns 401 if cookie not found', async () => {
            // Arrange
            const expectedResponseJson = { error: UNAUTHORIZED_ERROR_MESSAGE };
            const expectedResponseInit: ResponseInit = {
                headers: CORS_HEADERS,
                status: constants.HTTP_STATUS_UNAUTHORIZED,
            };
            const expectedResponse: Response = Response.json(
                expectedResponseJson,
                expectedResponseInit,
            );
            mockGetCookies.mockReturnValue(null);

            const authDecorators = createAuthDecorators(expectedAuthDecoratorsDeps);
            const decoratedController = authDecorators.withAuth(mockController);
            // Act
            const actualResponse: Response = await decoratedController(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(mockGetCookies).toHaveBeenLastCalledWith(USER_SESSION_COOKIE_NAME);
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
            expect(actualResponse.status).toEqual(expectedResponse.status);
        });

        it('returns 401 if session not found', async () => {
            // Arrange
            const expectedToken = 'token';
            const expectedResponseJson = { error: UNAUTHORIZED_ERROR_MESSAGE };
            const expectedResponseInit: ResponseInit = {
                headers: CORS_HEADERS,
                status: constants.HTTP_STATUS_UNAUTHORIZED,
            };
            const expectedResponse: Response = Response.json(
                expectedResponseJson,
                expectedResponseInit,
            );
            mockGetCookies.mockReturnValue(expectedToken);
            mockGetUserSession.mockImplementation(() => {
                throw new SessionNotFoundError();
            });

            const authDecorators = createAuthDecorators(expectedAuthDecoratorsDeps);
            const decoratedController = authDecorators.withAuth(mockController);
            // Act
            const actualResponse: Response = await decoratedController(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(mockGetCookies).toHaveBeenLastCalledWith(USER_SESSION_COOKIE_NAME);
            expect(mockGetUserSession).toHaveBeenLastCalledWith(expectedToken);
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
            expect(actualResponse.status).toEqual(expectedResponse.status);
        });
    });
});
