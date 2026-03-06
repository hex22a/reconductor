import createRegisterRoutes from './register';
import createLoginRoutes from './login';
import { createMeRoutes } from './me';

const registerRoutes = createRegisterRoutes();
const loginRoutes = createLoginRoutes();
const meRoutes = createMeRoutes();

export default { ...registerRoutes, ...loginRoutes, ...meRoutes };
