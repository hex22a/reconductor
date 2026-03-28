import { afterEach, describe, expect, mock, test, it } from 'bun:test';
import { createGraphQlContext, type GraphQlContextFactoryDeps } from './context';
import type { SessionRepository } from '../persistence/session.kv';
import type { GraphQlServerContext } from './server';
import type { GraphQlContext } from '../transport/graphql.context';
import type { UserSession } from '../domain/session.entity';
import {
    GRAPHQL_ERROR_EXTENSION_CODE,
    GRAPHQL_UNAUTHORIZED_ERROR_MESSAGE,
    USER_SESSION_COOKIE_NAME,
} from '../constants';
import type { BunRequest, CookieMap } from 'bun';
import { GraphQLError } from 'graphql/error';
import { SessionNotFoundError } from '../domain/errors/SessionNotFoundError';

describe('context', () => {
    const mockCreateUserSession = mock();
    const mockGetUserSession = mock();
    const mockDeleteUserSession = mock();
    const mockGetCookies = mock();
    const mockSessionRepository: SessionRepository = {
        createUserSession: mockCreateUserSession,
        getUserSession: mockGetUserSession,
        deleteUserSession: mockDeleteUserSession,
    };
    const mockCookies = {
        get: mockGetCookies,
    } satisfies Partial<CookieMap>;
    const mockRequest = {
        cookies: mockCookies as unknown as CookieMap,
    } satisfies Partial<BunRequest>;
    const expectedGraphQlContextDeps: GraphQlContextFactoryDeps = {
        sessionRepository: mockSessionRepository,
    };

    afterEach(() => {
        mockCreateUserSession.mockReset();
        mockGetUserSession.mockReset();
        mockDeleteUserSession.mockReset();
        mockGetCookies.mockReset();
    });

    test('createGraphQlContext', () => {
        // Arrange
        // Act
        const actualGraphQlContext = createGraphQlContext(expectedGraphQlContextDeps);
        // Assert
        expect(actualGraphQlContext).toBeFunction();
    });

    describe('graphQlContext', () => {
        it('returns user context if user session is found', async () => {
            // Arrange
            const expectedUserId = '019c94f3-4826-7c85-a663-98055fe5cba4';
            const expectedUsername = 'username';
            const expectedCookieToken = 'token';
            const expectedUserSession: UserSession = {
                userId: expectedUserId,
                username: expectedUsername,
                token: expectedCookieToken,
            };
            const expectedContext: GraphQlContext = {
                user: { id: expectedUserId },
            };
            const expectedServerContext: GraphQlServerContext = {
                request: mockRequest as unknown as BunRequest,
            };
            mockGetCookies.mockReturnValue(expectedCookieToken);
            mockGetUserSession.mockResolvedValue(expectedUserSession);
            const graphQlContext = createGraphQlContext(expectedGraphQlContextDeps);
            // Act
            const actualContext = await graphQlContext(expectedServerContext);
            // Assert
            expect(actualContext).toEqual(expectedContext);
            expect(mockGetCookies).toHaveBeenLastCalledWith(USER_SESSION_COOKIE_NAME);
            expect(mockGetUserSession).toHaveBeenLastCalledWith(expectedCookieToken);
        });

        it('throws an error if cookie not found', async () => {
            // Arrange
            const expectedServerContext: GraphQlServerContext = {
                request: mockRequest as unknown as BunRequest,
            };
            mockGetCookies.mockReturnValue(null);
            const graphQlContext = createGraphQlContext(expectedGraphQlContextDeps);
            // Act
            try {
                await graphQlContext(expectedServerContext);
            } catch (actualError) {
                // Assert
                expect(actualError).toBeInstanceOf(GraphQLError);
                expect((actualError as GraphQLError).message).toEqual(
                    GRAPHQL_UNAUTHORIZED_ERROR_MESSAGE,
                );
                expect((actualError as GraphQLError).extensions.code).toEqual(
                    GRAPHQL_ERROR_EXTENSION_CODE,
                );
                expect(mockGetCookies).toHaveBeenLastCalledWith(USER_SESSION_COOKIE_NAME);
            }
        });

        it('throws an error if session not found', async () => {
            // Arrange
            const expectedCookieToken = 'token';
            const expectedServerContext: GraphQlServerContext = {
                request: mockRequest as unknown as BunRequest,
            };
            mockGetCookies.mockReturnValue(expectedCookieToken);
            mockGetUserSession.mockImplementation(() => {
                throw new SessionNotFoundError();
            });
            const graphQlContext = createGraphQlContext(expectedGraphQlContextDeps);
            // Act
            try {
                await graphQlContext(expectedServerContext);
            } catch (actualError) {
                // Assert
                expect(actualError).toBeInstanceOf(GraphQLError);
                expect((actualError as GraphQLError).message).toEqual(
                    GRAPHQL_UNAUTHORIZED_ERROR_MESSAGE,
                );
                expect((actualError as GraphQLError).extensions.code).toEqual(
                    GRAPHQL_ERROR_EXTENSION_CODE,
                );
                expect(mockGetCookies).toHaveBeenLastCalledWith(USER_SESSION_COOKIE_NAME);
                expect(mockGetUserSession).toHaveBeenLastCalledWith(expectedCookieToken);
            }
        });
    });
});
