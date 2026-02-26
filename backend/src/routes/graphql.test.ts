import { describe, expect, mock, test } from 'bun:test';
import { createGraphQlRoutes } from './graphql';
import { GRAPHQL_ENDPOINT } from '../constants';
import container from '../container';
import { asFunction } from 'awilix';
import type { AuthDecorators } from '../decorators/auth';

describe('graphql', () => {
    test('createGraphQlRoutes', () => {
        // Arrange
        const mockDecoratedHandler = mock();
        const mockGraphQlServer = {
            fetch: mock(),
        };
        const mockAuthDecorators: AuthDecorators = {
            withAuth: mock().mockReturnValue(mockDecoratedHandler),
        };
        const mockGetGraphQlServerInstance = mock().mockReturnValue(mockGraphQlServer);
        const mockCreateAuthDecorators = mock().mockReturnValue(mockAuthDecorators);
        container.register({
            graphQlServer: asFunction(mockGetGraphQlServerInstance).singleton(),
            authDecorators: asFunction(mockCreateAuthDecorators).singleton(),
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
        expect(mockAuthDecorators.withAuth).toHaveBeenCalledWith(mockGraphQlServer.fetch);
    });
});
