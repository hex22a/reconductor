import type { UserInsert } from "../domain/user.entity";
import type { SQL } from "./db";

export type UserRepository = {
    addUser: (userInsert: UserInsert) => Promise<void>;
};

export function createUserRepository(sql: SQL): UserRepository {
    return {
        async addUser() {
            throw new Error('not implemented');
        }
    }
}
