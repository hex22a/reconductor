import createRegisterRoutes from './register';
import createLoginRoutes from './login';

const registerRoutes = createRegisterRoutes();
const loginRoutes = createLoginRoutes();

export default { ...registerRoutes, ...loginRoutes };
