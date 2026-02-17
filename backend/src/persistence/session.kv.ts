import { USER_SESSION_PREFIX } from "../constants";
import type { UserSession, UserSessionInsert } from "../domain/session.entity";
import type { KvClinent } from "./kv";

export type SessionRepository = {
    createUserSession: (userSession: UserSessionInsert) => Promise<UserSession>;
};

export function createSessionRepository(kv: KvClinent): SessionRepository {
    return {
        async createUserSession(userSession: UserSessionInsert): Promise<UserSession> {
            const { token, userId } = userSession;
            await kv.set(USER_SESSION_PREFIX + token, userId);
            return { userId };
        }
    }
}
