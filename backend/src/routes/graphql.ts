import { GRAPHQL_ENDPOINT } from '../constants';
import container from '../container';
import type { GraphQlServerInstance } from '@/src/graphql/server';
import type { AuthDecorators, AuthDecoratorsFactory } from '@/src/auth/decorators/auth';
import type { SessionStrategy } from '../auth/strategies/SessionStrategy';
import type { HandleCallStrategy } from '../auth/strategies/HandleCallStrategy';
import type { FetchToHandlerAdapter } from '../graphql/adapters/FetchToHandlerAdapter';
import type { RequestHandler } from '../controllers/types';
import type { WithCorsDecorator } from '../controllers/decorators/controller';

export function createGraphQlRoutes() {
    const graphQlServer: GraphQlServerInstance = container.resolve('graphQlServer');
    const sessionStrategy: SessionStrategy = container.resolve('sessionStrategy');
    const handleCallStrategy: HandleCallStrategy = container.resolve('handleCallStrategy');
    const createAuthDeocrators: AuthDecoratorsFactory = container.resolve('createAuthDecorators');
    const preflight: RequestHandler<void> = container.resolve('preflightController');
    const withCors: WithCorsDecorator = container.resolve('withCors');
    const toHandler: FetchToHandlerAdapter = container.resolve('fetchToHandlerAdapter');
    const { withAuth }: AuthDecorators<void> = createAuthDeocrators({
        authStrategy: sessionStrategy,
        handleStrategy: handleCallStrategy,
    });
    const decoratedGraphQlHandler: RequestHandler<void> = withCors(
        withAuth(toHandler(graphQlServer.fetch)),
    );
    return {
        [GRAPHQL_ENDPOINT]: {
            GET: decoratedGraphQlHandler,
            POST: decoratedGraphQlHandler,
            OPTIONS: withCors(preflight),
        },
    };
}
