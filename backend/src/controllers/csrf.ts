import type { BunRequest } from 'bun';
import type { CsrfRepository } from '../persistence/csrf.kv';
import type { SessionRepository } from '../persistence/session.kv';
import type { CsrfProvider } from '../providers/csrf';
import type { RequestHandler } from './types';
import type { UserSession } from '../domain/session.entity';
import { USER_SESSION_COOKIE_NAME } from '../constants';
import { SessionNotFoundError } from '../domain/errors/SessionNotFoundError';

export type CsrfControllerDeps = {
    sessionRepository: SessionRepository;
    csrfRepository: CsrfRepository;
    csrfProvider: CsrfProvider;
};

export type CsrfController = {
    getToken: RequestHandler<void>;
};

export function createCsrfController({
    sessionRepository,
    csrfRepository,
    csrfProvider,
}: CsrfControllerDeps): CsrfController {
    return {
        async getToken(req: BunRequest): Promise<Response> {
            let csrfToken;
            const sessionToken = req.cookies.get(USER_SESSION_COOKIE_NAME);
            if (sessionToken) {
                try {
                    const session: UserSession =
                        await sessionRepository.getUserSession(sessionToken);
                    csrfToken = session.csrfToken;
                } catch (error) {
                    if (error instanceof SessionNotFoundError) {
                        req.cookies.delete(USER_SESSION_COOKIE_NAME);
                        csrfToken = csrfProvider.generate();
                        csrfRepository.createAnonymousCsrf(csrfToken);
                    } else {
                        throw error;
                    }
                }
            } else {
                csrfToken = csrfProvider.generate();
                csrfRepository.createAnonymousCsrf(csrfToken);
            }
            return Response.json({ csrfToken });
        },
    };
}
