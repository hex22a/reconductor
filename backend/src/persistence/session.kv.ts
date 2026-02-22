import { USER_SESSION_PREFIX, USER_SESSION_TTL_SECONDS } from "../constants";
import { SessionNotFoundError } from "../domain/errors/SessionNotFoundError";
import type { UserSession, UserSessionInsert } from "../domain/session.entity";
import type { KvClinent } from "./kv";

export type SessionRepository = {
    createUserSession: (userSession: UserSessionInsert) => Promise<UserSession>;
    getUserSession: (token: string) => Promise<UserSession>;
};

export function createSessionRepository(kv: KvClinent): SessionRepository {
    return {
        async createUserSession(userSession: UserSessionInsert): Promise<UserSession> {
            const { token, userId, username } = userSession;
            const key = `${USER_SESSION_PREFIX}:${token}`;
            await kv.hset(key, userSession);
            await kv.expire(key, USER_SESSION_TTL_SECONDS);
            return { userId, username };
        },
        async getUserSession(token: string): Promise<UserSession> {
            const key = `${USER_SESSION_PREFIX}:${token}`;
            const userSession = await kv.hgetall(key);
            if (!userSession || !userSession.userId || !userSession.username) {
                throw new SessionNotFoundError();
            }
            return { userId: userSession.userId, username: userSession.username };
        }
    }
}
