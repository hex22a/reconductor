import type { BunRequest, MaybePromise } from 'bun';
import { constants } from 'node:http2';
import type { SessionRepository } from '../persistence/session.kv';
import { HEADERS, UNAUTHORIZED_ERROR_MESSAGE, USER_SESSION_COOKIE_NAME } from '../constants';

type RequestHandler = (req: BunRequest) => MaybePromise<Response>;

export type AuthDecorators = {
    withAuth: (handler: RequestHandler) => RequestHandler;
};

export type AuthDecoratorsFactoryDeps = {
    sessionRepository: SessionRepository;
};

export function createAuthDecorators({
    sessionRepository,
}: AuthDecoratorsFactoryDeps): AuthDecorators {
    return {
        withAuth(handler: RequestHandler): RequestHandler {
            return async function (req: BunRequest): Promise<Response> {
                const token = req.cookies.get(USER_SESSION_COOKIE_NAME);
                if (!token) {
                    return Response.json(
                        { error: UNAUTHORIZED_ERROR_MESSAGE },
                        { headers: HEADERS, status: constants.HTTP_STATUS_UNAUTHORIZED },
                    );
                }
                try {
                    await sessionRepository.getUserSession(token);
                    return handler(req);
                } catch {
                    return Response.json(
                        { error: UNAUTHORIZED_ERROR_MESSAGE },
                        { headers: HEADERS, status: constants.HTTP_STATUS_UNAUTHORIZED },
                    );
                }
            };
        },
    };
}
