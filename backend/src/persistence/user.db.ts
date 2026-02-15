import type { UserEntity, UserInsert } from "../domain/user.entity";
import type { SQL } from "./db";

export type UserRepository = {
    addUser: (userInsert: UserInsert) => Promise<UserEntity>;
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
        }
    }
}
