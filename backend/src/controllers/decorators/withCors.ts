import type { BunRequest } from 'bun';
import {
    ACCESS_CONTROL_ALLOW_CREDENTIALS_HEADER,
    ACCESS_CONTROL_ALLOW_HEADERS_HEADER,
    ACCESS_CONTROL_ALLOW_METHODS_HEADER,
    ACCESS_CONTROL_ALLOW_ORIGIN_HEADER,
    CORS_ALLOW_CREDENTIALS,
    CORS_ALLOWED_HEADERS,
    CORS_ALLOWED_METHODS,
    DASHBOARD_URL,
} from '@/src/constants';
import type { RequestHandler } from '@/src/controllers/types';

export function withCors(controller: RequestHandler<void>): RequestHandler<void> {
    return async function (req: BunRequest): Promise<Response> {
        const response: Response = await controller(req);
        response.headers.set(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER, DASHBOARD_URL);
        response.headers.set(ACCESS_CONTROL_ALLOW_METHODS_HEADER, CORS_ALLOWED_METHODS);
        response.headers.set(ACCESS_CONTROL_ALLOW_HEADERS_HEADER, CORS_ALLOWED_HEADERS);
        response.headers.set(ACCESS_CONTROL_ALLOW_CREDENTIALS_HEADER, CORS_ALLOW_CREDENTIALS);
        return response;
    };
}

export type WithCorsDecorator = typeof withCors;
