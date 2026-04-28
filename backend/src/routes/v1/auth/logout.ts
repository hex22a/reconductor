import type { AuthDecorators, AuthDecoratorsFactory } from '@/src/auth/decorators/auth';
import type { SessionStrategy } from '@/src/auth/strategies/SessionStrategy';
import { API_LOGOUT_ENDPOINT_V1 } from '@/src/constants';
import container from '@/src/container';
import type { WithCorsDecorator } from '@/src/controllers/decorators/withCors';
import type { RequestContext, RequestHandler } from '@/src/controllers/types';

export function createLogoutRoutes() {
    const logout: RequestHandler<RequestContext> = container.resolve('logoutController');
    const preflight: RequestHandler<void> = container.resolve('preflightController');
    const sessionStrategy: SessionStrategy = container.resolve('sessionStrategy');
    const createAuthDecorators: AuthDecoratorsFactory = container.resolve('createAuthDecorators');
    const withCors: WithCorsDecorator = container.resolve('withCors');
    const authDecorators: AuthDecorators<RequestContext> = createAuthDecorators({
        authStrategy: sessionStrategy,
    });
    return {
        [API_LOGOUT_ENDPOINT_V1]: {
            POST: withCors(authDecorators.withAuth(logout)),
            OPTIONS: withCors(preflight),
        },
    };
}
