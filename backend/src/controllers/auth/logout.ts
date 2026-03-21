import type { SessionRepository } from '@/src/persistence/session.kv';
import type { RequestContext, RequestHandler } from '../types';
import type { BunRequest } from 'bun';
import { constants } from 'node:http2';
import { USER_SESSION_COOKIE_NAME } from '@/src/constants';

export type LogoutControllerDeps = {
    sessionRepository: SessionRepository;
};

export function createLogoutController({
    sessionRepository,
}: LogoutControllerDeps): RequestHandler<RequestContext> {
    return async function logout(request: BunRequest, ctx: RequestContext): Promise<Response> {
        await sessionRepository.deleteUserSession(ctx.user.token);
        request.cookies.delete(USER_SESSION_COOKIE_NAME);
        return new Response(null, { status: constants.HTTP_STATUS_NO_CONTENT });
    };
}
