import type { BunRequest } from 'bun';
import type { ICsrfStrategy } from '../strategies/ICsrfStrategy';
import type { RequestContext, RequestHandler } from '../types';
import { FORBIDDEN_ERROR_MESSAGE } from '@/src/constants';
import { constants } from 'node:http2';

export type CsrfDecorators<Context extends RequestContext | void> = {
    withCsrf: (handler: RequestHandler<Context>) => RequestHandler<Context>;
};

export type CsrfDecoratorsFactoryDeps<Context extends RequestContext | void> = {
    csrfStrategy: ICsrfStrategy<Context>;
};

export function createCsrfDecorators<Context extends RequestContext | void>({
    csrfStrategy,
}: CsrfDecoratorsFactoryDeps<Context>): CsrfDecorators<Context> {
    return {
        withCsrf(handler: RequestHandler<Context>): RequestHandler<Context> {
            return async function (req: BunRequest, context: Context) {
                if (csrfStrategy.verifyCsrfToken(req, context)) {
                    return handler(req, context);
                } else {
                    return Response.json(
                        { error: FORBIDDEN_ERROR_MESSAGE },
                        { status: constants.HTTP_STATUS_FORBIDDEN },
                    );
                }
            };
        },
    };
}

export type CsrfDecoratorsFactory = typeof createCsrfDecorators;
