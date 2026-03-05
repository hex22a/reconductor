import type { RegisterController } from '@/src/controllers/auth/register';
import { API_REGISTER_ENDPOINT_V1 } from '../../../constants';
import container from '@/src/container';
import type { WithCorsDecorator, WithErrorHandlingDecorator } from '@/src/decorators/controller';
import type { RequestHandler } from '@/src/controllers/types';

export default function createRegisterRoutes() {
    const registerController: RegisterController = container.resolve('registerController');
    const withErrorHandling: WithErrorHandlingDecorator = container.resolve('withErrorHandling');
    const withCors: WithCorsDecorator = container.resolve('withCors');
    const preflight: RequestHandler = container.resolve('preflightController');
    return {
        [API_REGISTER_ENDPOINT_V1]: {
            POST: withCors(withErrorHandling(registerController.post)),
            OPTIONS: withCors(preflight),
        },
    };
}
