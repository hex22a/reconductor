import { USER_SESSION_EXPIRATION_MILLS, USER_SESSION_PREFIX } from "../constants";
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
            const { token, userId } = userSession;
            await kv.set(USER_SESSION_PREFIX + token, userId, 'PX', USER_SESSION_EXPIRATION_MILLS);
            return { userId };
        },
        async getUserSession(token: string): Promise<UserSession> {
            const userId = await kv.get(USER_SESSION_PREFIX + token);
            if (!userId) {
                throw new SessionNotFoundError();
            }
            return { userId };
        }
    }
}
