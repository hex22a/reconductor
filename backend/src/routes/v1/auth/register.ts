import type { RegisterController } from '@/src/controllers/auth/register';
import { API_REGISTER_ENDPOINT_V1 } from '../../../constants';
import container from '@/src/container';

export default function createRegisterRoutes() {
    const registerController: RegisterController = container.resolve('registerController');
    return {
        [API_REGISTER_ENDPOINT_V1]: {
            POST: registerController.post,
        }
    };
}; 
