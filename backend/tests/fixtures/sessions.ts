import type { UserSession, UserSessionInsert } from "@/src/domain/session.entity";

export function createUserSessionFixture(
    token: string,
    userId: string,
    username: string,
): [UserSessionInsert, UserSession] {
    return [
        {
            token,
            userId,
            username,
        },
        {
            userId,
            username,
        }
    ];
}
