import { withErrorHandling } from '@/src/decorators/controller';
import createRegisterRoutes from './register';
import createLoginRoutes from './login';

const registerRoutes = createRegisterRoutes(withErrorHandling);
const loginRoutes = createLoginRoutes(withErrorHandling);

export default { ...registerRoutes, ...loginRoutes };
