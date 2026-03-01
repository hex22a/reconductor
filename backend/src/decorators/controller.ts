import type { BunRequest, MaybePromise } from 'bun';
import { z, ZodError } from 'zod';
import {
    ACCESS_CONTROL_ALLOW_CREDENTIALS_HEADER,
    ACCESS_CONTROL_ALLOW_HEADERS_HEADER,
    ACCESS_CONTROL_ALLOW_METHODS_HEADER,
    ACCESS_CONTROL_ALLOW_ORIGIN_HEADER,
    CORS_ALLOW_CREDENTIALS,
    CORS_ALLOWED_HEADERS,
    CORS_ALLOWED_METHODS,
    DASHBOARD_URL,
    UNEXPECTED_ERROR_MESSAGE,
} from '../constants';
import { constants } from 'node:http2';
import {
    DATABASE_ERROR_CODE,
    SYNTAX_ERROR_CODE,
    UNEXPECTED_ERROR_CODE,
    VALIDATION_ERROR_CODE,
} from '$/constants';

export type RequestHandler = (req: BunRequest) => MaybePromise<Response>;

export function withErrorHandling(controller: RequestHandler): RequestHandler {
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
                return Response.json(
                    { code: DATABASE_ERROR_CODE, error: error.message },
                    { status: constants.HTTP_STATUS_BAD_REQUEST },
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

export function withCors(controller: RequestHandler): RequestHandler {
    return async function (req: BunRequest): Promise<Response> {
        const response: Response = await controller(req);
        response.headers.append(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER, DASHBOARD_URL);
        response.headers.append(ACCESS_CONTROL_ALLOW_METHODS_HEADER, CORS_ALLOWED_METHODS);
        response.headers.append(ACCESS_CONTROL_ALLOW_HEADERS_HEADER, CORS_ALLOWED_HEADERS);
        response.headers.append(ACCESS_CONTROL_ALLOW_CREDENTIALS_HEADER, CORS_ALLOW_CREDENTIALS);
        return response;
    };
}

export type WithCorsDecorator = typeof withCors;
