import type { SQL } from '@/src/persistence/db';
import {
    createProjectFixture,
    expectedExistingProjectId,
    expectedExistingProjectName,
} from '@/tests/fixtures/projects';
import {
    createScanFixture,
    expectedExistingScanId,
    expectedScanTarget,
} from '@/tests/fixtures/scans';
import {
    createUserFixture,
    expectedExistingUserId,
    expectedExistingUserPasswordHash,
    expectedExistingUserUsername,
} from '@/tests/fixtures/users';

async function seedUsers(sql: SQL): Promise<void> {
    const [, , rootUser] = createUserFixture(
        expectedExistingUserUsername,
        expectedExistingUserPasswordHash,
        expectedExistingUserId,
    );
    await sql`
        INSERT INTO recon.users
            (id, username, password_hash)
        VALUES
            (${rootUser.id}, ${rootUser.username}, ${rootUser.password_hash});
    `;
}

async function seedProjects(sql: SQL): Promise<void> {
    const [, , project] = createProjectFixture(
        expectedExistingProjectName,
        expectedExistingUserId,
        expectedExistingProjectId,
    );
    await sql`
        INSERT INTO recon.projects
            (id, name, owner_id)
        VALUES
            (${project.id}, ${project.name}, ${project.owner_id});
    `;
}

async function seedScans(sql: SQL): Promise<void> {
    const [, , scan] = createScanFixture(
        expectedExistingProjectId,
        expectedScanTarget,
        undefined,
        expectedExistingScanId,
    );
    await sql`
        INSERT INTO recon.scans
            (id, project_id, target)
        VALUES
            (${scan.id}, ${scan.project_id}, ${scan.target});
    `;
}

export async function seedDb(sql: SQL): Promise<void> {
    await seedUsers(sql);
    await seedProjects(sql);
    await seedScans(sql);
}
