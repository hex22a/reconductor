import createRegisterRoutes from './register';
import createLoginRoutes from './login';
import { createMeRoutes } from './me';
import { createLogoutRoutes } from './logout';

const registerRoutes = createRegisterRoutes();
const loginRoutes = createLoginRoutes();
const logoutRoutes = createLogoutRoutes();
const meRoutes = createMeRoutes();

export default { ...registerRoutes, ...loginRoutes, ...logoutRoutes, ...meRoutes };
