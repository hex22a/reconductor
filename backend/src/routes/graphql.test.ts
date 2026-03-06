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
        const mockDecoratedHandler = mock();
        const expectedSessionStrategy: SessionStrategy = {} as unknown as SessionStrategy;
        const expectedHandleCallStrategy: HandleCallStrategy = {} as unknown as HandleCallStrategy;
        const expectedAuthDecoratorsDeps: AuthDecoratorsFactoryDeps = {
            authStrategy: expectedSessionStrategy,
            handleStrategy: expectedHandleCallStrategy,
        };
        const mockGraphQlServer = {
            fetch: mock(),
        };
        const mockAuthDecorators: AuthDecorators = {
            withAuth: mock().mockReturnValue(mockDecoratedHandler),
        };
        const mockGetGraphQlServerInstance = mock().mockReturnValue(mockGraphQlServer);
        const mockCreateAuthDecorators = mock().mockReturnValue(mockAuthDecorators);
        container.register({
            sessionStrategy: asValue(expectedSessionStrategy),
            handleCallStrategy: asValue(expectedHandleCallStrategy),
            graphQlServer: asFunction(mockGetGraphQlServerInstance).singleton(),
            createAuthDecorators: asValue(mockCreateAuthDecorators),
        });
        const expectedRoutes = {
            [GRAPHQL_ENDPOINT]: {
                POST: mockDecoratedHandler,
            },
        };
        // Act
        const actualRoutes = createGraphQlRoutes();
        // Assert
        expect(actualRoutes).toEqual(expectedRoutes);
        expect(mockCreateAuthDecorators).toHaveBeenCalledWith(expectedAuthDecoratorsDeps);
        expect(mockAuthDecorators.withAuth).toHaveBeenCalledWith(mockGraphQlServer.fetch);
    });
});
