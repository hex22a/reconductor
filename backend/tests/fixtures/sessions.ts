import type { UserSession, UserSessionInsert } from '@/src/domain/session.entity';

export function createUserSessionFixture(
    token: string,
    userId: string,
    username: string,
): [UserSession, UserSessionInsert] {
    return [
        {
            userId,
            username,
        },
        {
            token,
            userId,
            username,
        },
    ];
}
