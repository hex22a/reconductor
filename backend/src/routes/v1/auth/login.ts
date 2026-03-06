import { API_LOGIN_ENDPOINT_V1 } from '@/src/constants';
import container from '@/src/container';
import type { LoginController } from '@/src/controllers/auth/login';
import type { RequestHandler } from '@/src/controllers/types';
import type {
    WithCorsDecorator,
    WithErrorHandlingDecorator,
} from '@/src/controllers/decorators/controller';

export default function createLoginRoutes() {
    const loginController: LoginController = container.resolve('loginController');
    const withErrorHandling: WithErrorHandlingDecorator = container.resolve('withErrorHandling');
    const withCors: WithCorsDecorator = container.resolve('withCors');
    const preflight: RequestHandler = container.resolve('preflightController');
    return {
        [API_LOGIN_ENDPOINT_V1]: {
            POST: withCors(withErrorHandling(loginController.post)),
            OPTIONS: withCors(preflight),
        },
    };
}
