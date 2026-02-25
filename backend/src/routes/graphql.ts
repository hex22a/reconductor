import { GRAPHQL_ENDPOINT } from '../constants';
import container from '../container';
import type { GraphQlServerInstance } from '../servers/graphql';

export function createGraphQlRoutes() {
    const graphQlServer: GraphQlServerInstance = container.resolve('graphQlServer');
    return {
        [GRAPHQL_ENDPOINT]: {
            POST: graphQlServer.fetch,
        },
    };
}
