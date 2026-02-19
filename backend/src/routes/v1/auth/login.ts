import { API_LOGIN_ENDPOINT_V1 } from '@/src/constants';
import container from '@/src/container';
import type { LoginController } from '@/src/controllers/auth/login';
import type { WithErrorHandlingDecorator } from '@/src/decorators/controller';

export default function createLoginRoutes(
    withErrorHandling: WithErrorHandlingDecorator
) {
    const loginController: LoginController = container.resolve('loginController');
    return {
        [API_LOGIN_ENDPOINT_V1]: {
            POST: withErrorHandling(loginController.post),
        }
    };
}; 
