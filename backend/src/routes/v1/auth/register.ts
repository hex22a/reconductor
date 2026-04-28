import type { RegisterController } from '@/src/controllers/auth/register';
import { API_REGISTER_ENDPOINT_V1 } from '../../../constants';
import container from '@/src/container';
import type { RequestHandler } from '@/src/controllers/types';
import type { WithErrorHandlingDecorator } from '@/src/controllers/decorators/withErrorHandling';
import type { WithCorsDecorator } from '@/src/controllers/decorators/withCors';

export default function createRegisterRoutes() {
    const registerController: RegisterController = container.resolve('registerController');
    const withErrorHandling: WithErrorHandlingDecorator = container.resolve('withErrorHandling');
    const withCors: WithCorsDecorator = container.resolve('withCors');
    const preflight: RequestHandler<void> = container.resolve('preflightController');
    return {
        [API_REGISTER_ENDPOINT_V1]: {
            POST: withCors(withErrorHandling(registerController.post)),
            OPTIONS: withCors(preflight),
        },
    };
}
