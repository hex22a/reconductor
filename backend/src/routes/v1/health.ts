import { API_HEALTH_ENDPOINT_V1 } from '@/src/constants';
import container from '@/src/container';
import type { WithCorsDecorator } from '@/src/controllers/decorators/controller';
import type { RequestHandler } from '@/src/controllers/types';

export default function createHealthRoutes() {
    const healthController: RequestHandler<void> = container.resolve('healthController');
    const withCors: WithCorsDecorator = container.resolve('withCors');
    const preflight: RequestHandler<void> = container.resolve('preflightController');
    return {
        [API_HEALTH_ENDPOINT_V1]: {
            GET: withCors(healthController),
            OPTIONS: withCors(preflight),
        },
    };
}
