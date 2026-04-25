import auth from './auth';
import { createCsrfRoutes } from './csrf';
import createHealthRoutes from './health';

const healthRoutes = createHealthRoutes();
const csrfRoutes = createCsrfRoutes();

export default { ...auth, ...healthRoutes, ...csrfRoutes };
