import { GRAPHQL_ENDPOINT } from '../constants';
import container from '../container';
import type { GraphQlServerInstance } from '@/src/graphql/server';
import type { AuthDecorators, AuthDecoratorsFactory } from '@/src/auth/decorators/auth';
import type { SessionStrategy } from '../auth/strategies/SessionStrategy';
import type { HandleCallStrategy } from '../auth/strategies/HandleCallStrategy';

export function createGraphQlRoutes() {
    const graphQlServer: GraphQlServerInstance = container.resolve('graphQlServer');
    const sessionStrategy: SessionStrategy = container.resolve('sessionStrategy');
    const handleCallStrategy: HandleCallStrategy = container.resolve('handleCallStrategy');
    const createAuthDeocrators: AuthDecoratorsFactory = container.resolve('createAuthDecorators');
    const { withAuth }: AuthDecorators = createAuthDeocrators({
        authStrategy: sessionStrategy,
        handleStrategy: handleCallStrategy,
    });
    return {
        [GRAPHQL_ENDPOINT]: {
            POST: withAuth(graphQlServer.fetch),
        },
    };
}
