import { API_CSRF_CONTROLLER_V1 } from '@/src/constants';
import container from '@/src/container';
import type { CsrfController } from '@/src/controllers/csrf';
import type {
    WithCorsDecorator,
    WithErrorHandlingDecorator,
} from '@/src/controllers/decorators/controller';
import type { RequestHandler } from '@/src/controllers/types';

export function createCsrfRoutes() {
    const csrfController: CsrfController = container.resolve('csrfController');
    const withErrorHandling: WithErrorHandlingDecorator = container.resolve('withErrorHandling');
    const withCors: WithCorsDecorator = container.resolve('withCors');
    const preflight: RequestHandler<void> = container.resolve('preflightController');
    return {
        [API_CSRF_CONTROLLER_V1]: {
            GET: withCors(withErrorHandling(csrfController.getToken)),
            OPTIONS: withCors(preflight),
        },
    };
}
