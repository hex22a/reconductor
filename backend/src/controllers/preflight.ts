import { constants } from 'node:http2';

export function preflight(): Response {
    return new Response(null, { status: constants.HTTP_STATUS_NO_CONTENT });
}
