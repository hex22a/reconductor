import { sql, type SQL } from '@/src/persistence/db';
import { createUserFixture, expectedExistingUserPasswordHash, expectedExistingUserUsername } from '@/tests/fixtures/users';

async function seedUsers(sql: SQL): Promise<void> {
    const [rootUser] = createUserFixture(expectedExistingUserUsername, expectedExistingUserPasswordHash);
    await sql`INSERT INTO recon.users (username, password_hash) VALUES (${rootUser.username}, ${rootUser.password_hash})`;
}

export async function seedDb(sql: SQL): Promise<void> {
    await seedUsers(sql);
}

