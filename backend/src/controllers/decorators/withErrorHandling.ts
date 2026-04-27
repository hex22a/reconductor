import type { BunRequest } from 'bun';
import type { RequestHandler } from '../types';
import z, { ZodError } from 'zod';
import {
    DATABASE_ERROR_CODE,
    SYNTAX_ERROR_CODE,
    UNEXPECTED_ERROR_CODE,
    VALIDATION_ERROR_CODE,
} from '$/constants';
import { DATABASE_ERROR_MESSAGE, UNEXPECTED_ERROR_MESSAGE } from '@/src/constants';
import { constants } from 'node:http2';

export function withErrorHandling(controller: RequestHandler<void>): RequestHandler<void> {
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
                        status: constants.HTTP_STATUS_UNPROCESSABLE_ENTITY,
                    },
                );
            }
            if (error instanceof SyntaxError) {
                return Response.json(
                    { code: SYNTAX_ERROR_CODE, error: error.message },
                    { status: constants.HTTP_STATUS_BAD_REQUEST },
                );
            }
            if (error instanceof Bun.SQL.PostgresError) {
                console.error(error.message);
                return Response.json(
                    { code: DATABASE_ERROR_CODE, error: DATABASE_ERROR_MESSAGE },
                    { status: constants.HTTP_STATUS_INTERNAL_SERVER_ERROR },
                );
            }
            console.error(error);
            return Response.json(
                { code: UNEXPECTED_ERROR_CODE, error: UNEXPECTED_ERROR_MESSAGE },
                { status: constants.HTTP_STATUS_INTERNAL_SERVER_ERROR },
            );
        }
    };
}

export type WithErrorHandlingDecorator = typeof withErrorHandling;
