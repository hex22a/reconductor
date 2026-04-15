import { constants } from 'node:http2';

export function health() {
    return Response.json(
        { healthy: true },
        {
            status: constants.HTTP_STATUS_OK,
        },
    );
}
