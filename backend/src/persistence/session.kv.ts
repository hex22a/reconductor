import { USER_SESSION_PREFIX, USER_SESSION_TTL_SECONDS } from '../constants';
import { SessionNotFoundError } from '../domain/errors/SessionNotFoundError';
import type { UserSession } from '../domain/session.entity';
import type { KvClinent } from './kv';

export type SessionRepositoryDeps = {
    kv: KvClinent;
};

export type SessionRepository = {
    createUserSession: (userSession: UserSession) => Promise<UserSession>;
    getUserSession: (token: string) => Promise<UserSession>;
    deleteUserSession: (token: string) => Promise<void>;
};

export function createSessionRepository({ kv }: SessionRepositoryDeps): SessionRepository {
    return {
        async createUserSession(userSession: UserSession): Promise<UserSession> {
            const { token, userId, username } = userSession;
            const key = `${USER_SESSION_PREFIX}:${token}`;
            await kv.hset(key, userSession);
            await kv.expire(key, USER_SESSION_TTL_SECONDS);
            return { token, userId, username };
        },
        async getUserSession(token: string): Promise<UserSession> {
            const key = `${USER_SESSION_PREFIX}:${token}`;
            const userSession = await kv.hgetall(key);
            if (!userSession || !userSession.userId || !userSession.username) {
                throw new SessionNotFoundError();
            }
            return { token, userId: userSession.userId, username: userSession.username };
        },
        async deleteUserSession(token: string): Promise<void> {
            const key = `${USER_SESSION_PREFIX}:${token}`;
            await kv.del(key);
        },
    };
}
