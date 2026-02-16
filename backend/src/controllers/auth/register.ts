import type { BunRequest } from "bun";
import { HEADERS, UNEXPECTED_ERROR_MESSAGE } from "../../constants";
import { constants } from 'node:http2';

export async function post(req: BunRequest): Promise<Response> {
    try {
        await req.json();
        return Response.json({ foo: 'bar' }, { headers: HEADERS });
    } catch (error) {
        if (error instanceof Error) {
            return Response.json(
                { error: error.message },
                { headers: HEADERS, status: constants.HTTP_STATUS_BAD_REQUEST }
            )
        }
        return Response.json(
            { error: UNEXPECTED_ERROR_MESSAGE },
            { headers: HEADERS, status: constants.HTTP_STATUS_INTERNAL_SERVER_ERROR }
        )
    }
}
