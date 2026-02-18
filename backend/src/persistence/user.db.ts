import { UserNotFoundError } from "../domain/errors/UserNotFoundError";
import type { UserEntity, UserInsert } from "../domain/user.entity";
import type { SQL } from "./db";

export type UserRepository = {
    addUser: (userInsert: UserInsert) => Promise<UserEntity>;
    getUserByUsername: (username: string) => Promise<UserEntity>;
};

export function createUserRepository(sql: SQL): UserRepository {
    return {
        async addUser(userInsert: UserInsert): Promise<UserEntity> {
            const queryResults = await sql`
                INSERT INTO recon.users
                    (username, password_hash)
                VALUES
                    (${userInsert.username}, ${userInsert.password_hash})
                RETURNING *;
            `;
            return queryResults[0];
        },
        async getUserByUsername(username: string): Promise<UserEntity> {
            const [user] = await sql<UserEntity[]>`
                SELECT
                    id,
                    username,
                    password_hash,
                    password_version,
                    created_at,
                    updated_at,
                    last_login_at,
                    is_active
                FROM recon.users
                WHERE username=${username}
                LIMIT 1;
            `;
            if (!user) {
                throw new UserNotFoundError();
            }
            return user;
        }
    }
}
