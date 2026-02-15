import type { UserInsert, UserEntity } from "@/src/domain/user.entity";

export const expectedExistingUserUsername = 'root';
export const expectedExistingUserPasswordHash = 'hash';

export function createUserFixture(username: string, passwordHash: string): [UserInsert, UserEntity] {
    const date = new Date();
    return [
        {
            username,
            password_hash: passwordHash
        },
        {
            id: Bun.randomUUIDv7(),
            username,
            password_hash: passwordHash,
            password_version: 1,
            created_at: date,
            updated_at: date,
            last_login_at: date,
            is_active: true,
        }
    ];
};
