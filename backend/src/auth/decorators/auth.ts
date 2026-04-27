import type { BunRequest } from 'bun';
import { constants } from 'node:http2';
import { UNAUTHORIZED_ERROR_MESSAGE } from '@/src/constants';
import type { RequestContext, RequestHandler } from '@/src/controllers/types';
import type { IAuthStrategy } from '../strategies/IAuthStrategy';
import type { IHandleStrategy } from '../../controllers/strategies/IHandleStrategy';

export type AuthDecorators<Context extends RequestContext | void> = {
    withAuth: (handler: RequestHandler<Context>) => RequestHandler<void>;
};

export type AuthDecoratorsFactoryDeps<Context extends RequestContext | void> = {
    authStrategy: IAuthStrategy;
    handleStrategy: IHandleStrategy<Context>;
};

export function createAuthDecorators<Context extends RequestContext | void>({
    authStrategy,
    handleStrategy,
}: AuthDecoratorsFactoryDeps<Context>): AuthDecorators<Context> {
    return {
        withAuth(handler: RequestHandler<Context>): RequestHandler<void> {
            return async function (req: BunRequest): Promise<Response> {
                try {
                    const userSession = await authStrategy.authenticate(req);
                    const context: RequestContext = {
                        user: userSession,
                    };
                    return handleStrategy.handle(handler, req, context);
                } catch {
                    return Response.json(
                        { error: UNAUTHORIZED_ERROR_MESSAGE },
                        { status: constants.HTTP_STATUS_UNAUTHORIZED },
                    );
                }
            };
        },
    };
}
export type AuthDecoratorsFactory = typeof createAuthDecorators;
