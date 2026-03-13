import { describe, expect, mock, test } from 'bun:test';
import { createGraphQlRoutes } from './graphql';
import { GRAPHQL_ENDPOINT } from '../constants';
import container from '../container';
import { asFunction, asValue } from 'awilix';
import type { AuthDecorators, AuthDecoratorsFactoryDeps } from '@/src/auth/decorators/auth';
import { SessionStrategy } from '../auth/strategies/SessionStrategy';
import type { HandleCallStrategy } from '../auth/strategies/HandleCallStrategy';

describe('graphql', () => {
    test('createGraphQlRoutes', () => {
        // Arrange
        const mockHandlerWithAuth = mock();
        const mockAdaptedHandler = mock();
        const mockControllerWithCors = mock();
        const mockPreflightController = mock();
        const expectedSessionStrategy: SessionStrategy = {} as unknown as SessionStrategy;
        const expectedHandleCallStrategy: HandleCallStrategy = {} as unknown as HandleCallStrategy;
        const expectedAuthDecoratorsDeps: AuthDecoratorsFactoryDeps<void> = {
            authStrategy: expectedSessionStrategy,
            handleStrategy: expectedHandleCallStrategy,
        };
        const mockGraphQlServer = {
            fetch: mock(),
        };
        const mockAuthDecorators: AuthDecorators<void> = {
            withAuth: mock().mockReturnValue(mockHandlerWithAuth),
        };
        const mockGetGraphQlServerInstance = mock().mockReturnValue(mockGraphQlServer);
        const mockCreateAuthDecorators = mock().mockReturnValue(mockAuthDecorators);
        const mockFetchToHandlerAdapter = mock().mockReturnValue(mockAdaptedHandler);
        const mockWithCors = mock().mockReturnValue(mockControllerWithCors);
        container.register({
            sessionStrategy: asValue(expectedSessionStrategy),
            handleCallStrategy: asValue(expectedHandleCallStrategy),
            graphQlServer: asFunction(mockGetGraphQlServerInstance).singleton(),
            createAuthDecorators: asValue(mockCreateAuthDecorators),
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
        expect(mockFetchToHandlerAdapter).toHaveBeenLastCalledWith(mockGraphQlServer.fetch);
        expect(mockAuthDecorators.withAuth).toHaveBeenCalledWith(mockAdaptedHandler);
        expect(mockWithCors).toHaveBeenNthCalledWith(1, mockHandlerWithAuth);
        expect(mockWithCors).toHaveBeenNthCalledWith(2, mockPreflightController);
    });
});
