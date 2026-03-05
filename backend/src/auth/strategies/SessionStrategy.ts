import type { BunRequest } from 'bun';
import type { IAuthStrategy } from './IAuthStrategy';
import type { SessionRepository } from '@/src/persistence/session.kv';
import type { UserSession } from '@/src/domain/session.entity';
import { USER_SESSION_COOKIE_NAME } from '@/src/constants';
import { UnauthorizedError } from '@/src/domain/errors/UnauthorizedError';

export type AuthStrategyDeps = {
    sessionRepository: SessionRepository;
};

export class SessionStrategy implements IAuthStrategy {
    sessionRepository: SessionRepository;
    constructor({ sessionRepository }: AuthStrategyDeps) {
        this.sessionRepository = sessionRepository;
    }
    authenticate(request: BunRequest): Promise<UserSession> {
        const token = request.cookies.get(USER_SESSION_COOKIE_NAME);
        if (!token) {
            throw new UnauthorizedError();
        }
        try {
            return this.sessionRepository.getUserSession(token);
        } catch {
            throw new UnauthorizedError();
        }
    }
}
