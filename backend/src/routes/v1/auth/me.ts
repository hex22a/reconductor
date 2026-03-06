import type { AuthDecorators, AuthDecoratorsFactory } from '@/src/auth/decorators/auth';
import type { HandleWithContextStrategy } from '@/src/auth/strategies/HandleWithContextStrategy';
import type { SessionStrategy } from '@/src/auth/strategies/SessionStrategy';
import { API_ME_ENDPOINT_V1 } from '@/src/constants';
import container from '@/src/container';
import type { WithCorsDecorator } from '@/src/controllers/decorators/controller';
import type { RequestContext, RequestHandler } from '@/src/controllers/types';

export function createMeRoutes() {
    const meController: RequestHandler<RequestContext> = container.resolve('meController');
    const preflight: RequestHandler = container.resolve('preflightController');
    const sessionStrategy: SessionStrategy = container.resolve('sessionStrategy');
    const handleWithContextStrategy: HandleWithContextStrategy = container.resolve(
        'handleWithContextStrategy',
    );
    const createAuthDecorators: AuthDecoratorsFactory = container.resolve('createAuthDecorators');
    const withCors: WithCorsDecorator = container.resolve('withCors');
    const authDecorators: AuthDecorators = createAuthDecorators({
        authStrategy: sessionStrategy,
        handleStrategy: handleWithContextStrategy,
    });
    return {
        [API_ME_ENDPOINT_V1]: {
            GET: withCors(authDecorators.withAuth(meController)),
            OPTIONS: withCors(preflight),
        },
    };
}
