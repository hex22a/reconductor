import type { UserInsert, UserEntity, UserInsertSeed } from '@/src/domain/user.entity';
import { randomUUIDv7 } from 'bun';

export const expectedExistingUserId = '6026c64e-5c35-402d-b6a5-f9b1306104df';
export const expectedExistingUserUsername = 'root';
export const expectedExistingUserPasswordHash = 'hash';

export function createUserFixture(
    username: string,
    passwordHash: string,
    userId?: string,
): [UserEntity, UserInsert, UserInsertSeed] {
    const id = userId || randomUUIDv7();
    const date = new Date();
    return [
        {
            id,
            username,
            password_hash: passwordHash,
            password_version: 1,
            created_at: date,
            updated_at: date,
            last_login_at: date,
            is_active: true,
        },
        {
            username,
            password_hash: passwordHash,
        },
        {
            id,
            username,
            password_hash: passwordHash,
        },
    ];
}
