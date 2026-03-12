import type { SessionRepository } from '@/src/persistence/session.kv';
import type { RequestContext, RequestHandler } from '../types';
import type { BunRequest } from 'bun';
import { constants } from 'node:http2';

export type LogoutControllerDeps = {
    sessionRepository: SessionRepository;
};

export function createLogoutController({
    sessionRepository,
}: LogoutControllerDeps): RequestHandler<RequestContext> {
    return async function logout(_: BunRequest, ctx: RequestContext): Promise<Response> {
        await sessionRepository.deleteUserSession(ctx.user.token);
        return new Response(null, { status: constants.HTTP_STATUS_NO_CONTENT });
    };
}
