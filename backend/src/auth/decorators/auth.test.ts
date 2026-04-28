import { describe, test, it, mock, afterEach, expect } from 'bun:test';
import { constants } from 'node:http2';
import type { BunRequest } from 'bun';
import { createAuthDecorators, type AuthDecorators, type AuthDecoratorsFactoryDeps } from './auth';
import { CORS_HEADERS, UNAUTHORIZED_ERROR_MESSAGE } from '@/src/constants';
import type { UserSession } from '@/src/domain/session.entity';
import type { IAuthStrategy } from '../strategies/IAuthStrategy';
import type { RequestContext } from '@/src/controllers/types';
import { UnauthorizedError } from '@/src/domain/errors/UnauthorizedError';

describe('auth', () => {
    class MockAuthStrategy implements IAuthStrategy {
        authenticate(): Promise<UserSession> {
            throw new Error('Method not implemented.');
        }
    }
    const mockAuthenticate = mock();
    const mockAuthStrategy = new MockAuthStrategy();
    mockAuthStrategy.authenticate = mockAuthenticate;
    const mockController = mock();
    const expectedRequest = {} satisfies Partial<BunRequest>;
    const expectedAuthDecoratorsDeps: AuthDecoratorsFactoryDeps = {
        authStrategy: mockAuthStrategy,
    };

    afterEach(() => {
        mockController.mockReset();
        mockAuthenticate.mockReset();
    });

    test('createAuthDecorators', () => {
        // Arrange
        const expectedAuthDecorators: AuthDecorators<RequestContext> = {
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
            const expectedCsrfToken = 'csrf_token';
            const expectedUserSession: UserSession = {
                token: expectedToken,
                userId: expectedUserId,
                username: expectedUsername,
                csrfToken: expectedCsrfToken,
            };
            const expectedContext: RequestContext = {
                user: expectedUserSession,
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
            mockAuthenticate.mockResolvedValue(expectedUserSession);

            const authDecorators = createAuthDecorators(expectedAuthDecoratorsDeps);
            const decoratedController = authDecorators.withAuth(mockController);
            // Act
            const actualResponse: Response = await decoratedController(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(mockAuthenticate).toHaveBeenCalledWith(expectedRequest);
            expect(mockController).toHaveBeenCalledWith(expectedRequest, expectedContext);
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
            expect(actualResponse.status).toEqual(expectedResponse.status);
        });

        it('returns 401 if auth strategy throws UnauthorizedError', async () => {
            // Arrange
            const expectedResponseJson = { error: UNAUTHORIZED_ERROR_MESSAGE };
            const expectedResponseInit: ResponseInit = {
                status: constants.HTTP_STATUS_UNAUTHORIZED,
            };
            const expectedResponse: Response = Response.json(
                expectedResponseJson,
                expectedResponseInit,
            );
            mockAuthenticate.mockImplementation(() => {
                throw new UnauthorizedError();
            });

            const authDecorators = createAuthDecorators(expectedAuthDecoratorsDeps);
            const decoratedController = authDecorators.withAuth(mockController);
            // Act
            const actualResponse: Response = await decoratedController(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(mockAuthenticate).toHaveBeenCalledWith(expectedRequest);
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
            expect(actualResponse.status).toEqual(expectedResponse.status);
        });
    });
});
