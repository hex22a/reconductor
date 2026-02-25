import type { RegisterController } from '@/src/controllers/auth/register';
import { API_REGISTER_ENDPOINT_V1 } from '../../../constants';
import container from '@/src/container';
import type { WithErrorHandlingDecorator } from '@/src/decorators/controller';

export default function createRegisterRoutes(withErrorHandling: WithErrorHandlingDecorator) {
    const registerController: RegisterController = container.resolve('registerController');
    return {
        [API_REGISTER_ENDPOINT_V1]: {
            POST: withErrorHandling(registerController.post),
        },
    };
}
