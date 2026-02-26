import type { BunRequest } from 'bun';
import { z, ZodError } from 'zod';
import { HEADERS, UNEXPECTED_ERROR_MESSAGE } from '../constants';
import { constants } from 'node:http2';
import {
    DATABASE_ERROR_CODE,
    SYNTAX_ERROR_CODE,
    UNEXPECTED_ERROR_CODE,
    VALIDATION_ERROR_CODE,
} from '$/constants';

export function withErrorHandling(
    controller: (req: BunRequest) => Promise<Response>,
): (res: BunRequest) => Promise<Response> {
    return async function (req: BunRequest): Promise<Response> {
        try {
            const response: Response = await controller(req);
            return response;
        } catch (error) {
            if (error instanceof ZodError) {
                const errorRespnse = z.flattenError(error);
                return Response.json(
                    {
                        code: VALIDATION_ERROR_CODE,
                        error: errorRespnse,
                    },
                    {
                        headers: HEADERS,
                        status: constants.HTTP_STATUS_UNPROCESSABLE_ENTITY,
                    },
                );
            }
            if (error instanceof SyntaxError) {
                return Response.json(
                    { code: SYNTAX_ERROR_CODE, error: error.message },
                    { headers: HEADERS, status: constants.HTTP_STATUS_BAD_REQUEST },
                );
            }
            if (error instanceof Bun.SQL.PostgresError) {
                return Response.json(
                    { code: DATABASE_ERROR_CODE, error: error.message },
                    { headers: HEADERS, status: constants.HTTP_STATUS_BAD_REQUEST },
                );
            }
            console.error(error);
            return Response.json(
                { code: UNEXPECTED_ERROR_CODE, error: UNEXPECTED_ERROR_MESSAGE },
                { headers: HEADERS, status: constants.HTTP_STATUS_INTERNAL_SERVER_ERROR },
            );
        }
    };
}

export type WithErrorHandlingDecorator = typeof withErrorHandling;
