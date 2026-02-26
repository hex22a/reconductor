import { GRAPHQL_ENDPOINT } from '../constants';
import container from '../container';
import type { GraphQlServerInstance } from '@/src/graphql/server';
import type { AuthDecorators } from '../decorators/auth';

export function createGraphQlRoutes() {
    const graphQlServer: GraphQlServerInstance = container.resolve('graphQlServer');
    const { withAuth }: AuthDecorators = container.resolve('authDecorators');
    return {
        [GRAPHQL_ENDPOINT]: {
            POST: withAuth(graphQlServer.fetch),
        },
    };
}
