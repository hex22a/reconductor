import { describe, expect, mock, test } from 'bun:test';
import { createGraphQlRoutes } from './graphql';
import { GRAPHQL_ENDPOINT } from '../constants';
import container from '../container';
import { asFunction, asValue } from 'awilix';
import type { AuthDecorators, AuthDecoratorsFactoryDeps } from '@/src/auth/decorators/auth';
import { SessionStrategy } from '../auth/strategies/SessionStrategy';
import type { SessionCsrfStrategy } from '../controllers/strategies/SessionCsrfStrategy';
import {
    type CsrfDecorators,
    type CsrfDecoratorsFactoryDeps,
} from '../controllers/decorators/withCsrf';
import type { RequestContext } from '../controllers/types';

describe('graphql', () => {
    test('createGraphQlRoutes', () => {
        // Arrange
        const mockHandlerWithAuth = mock();
        const mockAdaptedHandler = mock();
        const mockControllerWithCors = mock();
        const mockControllerWithCsrf = mock();
        const mockPreflightController = mock();
        const mockWithCsrf = mock().mockReturnValue(mockControllerWithCsrf);
        const expectedSessionStrategy: SessionStrategy = {} as unknown as SessionStrategy;
        const expectedSessionCsrfStrategy: SessionCsrfStrategy =
            {} as unknown as SessionCsrfStrategy;
        const expectedAuthDecoratorsDeps: AuthDecoratorsFactoryDeps = {
            authStrategy: expectedSessionStrategy,
        };
        const mockCsrfDecoratorFactoryDeps: CsrfDecoratorsFactoryDeps<RequestContext> = {
            csrfStrategy: expectedSessionCsrfStrategy,
        };
        const mockGraphQlServer = {
            fetch: mock(),
        };
        const mockAuthDecorators: AuthDecorators<void> = {
            withAuth: mock().mockReturnValue(mockHandlerWithAuth),
        };
        const mockCsrfDecorators: CsrfDecorators<RequestContext> = {
            withCsrf: mockWithCsrf,
        };
        const mockGetGraphQlServerInstance = mock().mockReturnValue(mockGraphQlServer);
        const mockCreateAuthDecorators = mock().mockReturnValue(mockAuthDecorators);
        const mockCreateCsrfDecorators = mock().mockReturnValue(mockCsrfDecorators);
        const mockFetchToHandlerAdapter = mock().mockReturnValue(mockAdaptedHandler);
        const mockWithCors = mock().mockReturnValue(mockControllerWithCors);
        container.register({
            sessionStrategy: asValue(expectedSessionStrategy),
            sessionCsrfStrategy: asValue(expectedSessionCsrfStrategy),
            graphQlServer: asFunction(mockGetGraphQlServerInstance).singleton(),
            createAuthDecorators: asValue(mockCreateAuthDecorators),
            createCsrfDecorators: asValue(mockCreateCsrfDecorators),
            fetchToHandlerAdapter: asValue(mockFetchToHandlerAdapter),
            preflightController: asValue(mockPreflightController),
            withCors: asValue(mockWithCors),
        });
        const expectedRoutes = {
            [GRAPHQL_ENDPOINT]: {
                GET: mockControllerWithCors,
                POST: mockControllerWithCors,
                OPTIONS: mockControllerWithCors,
            },
        };
        // Act
        const actualRoutes = createGraphQlRoutes();
        // Assert
        expect(actualRoutes).toEqual(expectedRoutes);
        expect(mockCreateAuthDecorators).toHaveBeenCalledWith(expectedAuthDecoratorsDeps);
        expect(mockCreateCsrfDecorators).toHaveBeenCalledWith(mockCsrfDecoratorFactoryDeps);
        expect(mockFetchToHandlerAdapter).toHaveBeenLastCalledWith(mockGraphQlServer.fetch);
        expect(mockWithCsrf).toHaveBeenCalledWith(mockAdaptedHandler);
        expect(mockAuthDecorators.withAuth).toHaveBeenCalledWith(mockControllerWithCsrf);
        expect(mockWithCors).toHaveBeenNthCalledWith(1, mockHandlerWithAuth);
        expect(mockWithCors).toHaveBeenNthCalledWith(2, mockPreflightController);
    });
});
