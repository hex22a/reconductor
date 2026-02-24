import { describe, expect, mock, test } from 'bun:test';
import { createGraphQlRoutes } from './graphql';
import { GRAPHQL_ENDPOINT } from '../constants';
import container from '../container';
import { asFunction } from 'awilix';

describe('graphql', () => {
    test('createGraphQlRoutes', () => {
        // Arrange
        const mockGraphQlServer = {
            fetch: mock(),
        };
        const mockGetGraphQlServerInstance = mock().mockReturnValue(mockGraphQlServer);
        container.register({
            graphQlServer: asFunction(mockGetGraphQlServerInstance).singleton(),
        });
        const expectedRoutes = {
            [GRAPHQL_ENDPOINT]: {
                POST: mockGraphQlServer.fetch,
            }
        };
        // Act
        const actualRoutes = createGraphQlRoutes();
        // Assert
        expect(actualRoutes).toEqual(expectedRoutes);
    });
});
