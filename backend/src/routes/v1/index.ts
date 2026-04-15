import auth from './auth';
import createHealthRoutes from './health';

const healthRoutes = createHealthRoutes();

export default { ...auth, ...healthRoutes };
