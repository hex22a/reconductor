import { describe, expect, mock, test } from 'bun:test';
import type { ProjectResolver } from './resolvers/project';
import type { GraphQLSchemaWithContext } from 'graphql-yoga';
import {
    getGraphQlServerInstance,
    type GraphQlServerFactoryDeps,
    type GraphQlServerInstance,
} from './server';

describe('graphql', () => {
    test('getGraphQlServerInstance', async () => {
        // Arrange
        const expectedGraphQlServerInstance = {} as unknown as GraphQlServerInstance;
        const expectedTypeDefs = await Bun.file('../shared/schema.graphql').text();
        const mockCreateGraphQlServer = mock();
        const mockCreateGraphQlSchema = mock();
        const mockProjectResolver: ProjectResolver = {
            Query: {
                project: mock(),
                projects: mock(),
            },
        };
        const expectedSchemaDefinition = {
            typeDefs: expectedTypeDefs,
            resolvers: mockProjectResolver,
        };
        const expectedSchema = {} satisfies Partial<GraphQLSchemaWithContext<never>>;
        const expectedServerOptions = {
            schema: expectedSchema,
        };
        mockCreateGraphQlSchema.mockReturnValue(expectedSchema);
        mockCreateGraphQlServer.mockReturnValue(expectedGraphQlServerInstance);
        const expectedGraphQlServerFactoryDeps: GraphQlServerFactoryDeps = {
            createGraphQlServer: mockCreateGraphQlServer,
            createGraphQlSchema: mockCreateGraphQlSchema,
            projectResolver: mockProjectResolver,
        };
        // Act
        const actualGraphQlServerInstance: GraphQlServerInstance = getGraphQlServerInstance(
            expectedGraphQlServerFactoryDeps,
        );
        // Assert
        expect(actualGraphQlServerInstance).toEqual(expectedGraphQlServerInstance);
        expect(mockCreateGraphQlServer).toHaveBeenLastCalledWith(expectedServerOptions);
        expect(mockCreateGraphQlSchema).toHaveBeenLastCalledWith(expectedSchemaDefinition);
    });
});
