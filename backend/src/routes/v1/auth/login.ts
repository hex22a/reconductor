import { API_LOGIN_ENDPOINT_V1 } from '@/src/constants';
import container from '@/src/container';
import type { LoginController } from '@/src/controllers/auth/login';
import type { RequestHandler } from '@/src/controllers/types';
import type { WithCorsDecorator } from '@/src/controllers/decorators/withCors';
import type { WithErrorHandlingDecorator } from '@/src/controllers/decorators/withErrorHandling';
import type { AnonymousCsrfStrategy } from '@/src/controllers/strategies/AnonymousCsrfStrategy';
import type { CsrfDecoratorsFactory } from '@/src/controllers/decorators/withCsrf';

export default function createLoginRoutes() {
    const loginController: LoginController = container.resolve('loginController');
    const withErrorHandling: WithErrorHandlingDecorator = container.resolve('withErrorHandling');
    const withCors: WithCorsDecorator = container.resolve('withCors');
    const anonymousCsrfStrategy: AnonymousCsrfStrategy = container.resolve('anonymousCsrfStrategy');
    const createCsrfDecorators: CsrfDecoratorsFactory = container.resolve('createCsrfDecorators');
    const preflight: RequestHandler<void> = container.resolve('preflightController');
    const { withCsrf } = createCsrfDecorators({
        csrfStrategy: anonymousCsrfStrategy,
    });
    return {
        [API_LOGIN_ENDPOINT_V1]: {
            POST: withCors(withErrorHandling(withCsrf(loginController.post))),
            OPTIONS: withCors(preflight),
        },
    };
}
