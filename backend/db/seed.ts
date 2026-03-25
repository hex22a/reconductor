import { expectedExistingHostId, expectedHostIp } from '@/src/domain/host.entity';
import { expectedExistingPortId, expectedPort } from '@/src/domain/port.entity';
import type { SQL } from '@/src/persistence/db';
import { createHostFixture } from '@/tests/fixtures/hosts';
import { createPortFixture } from '@/tests/fixtures/ports';
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

async function seedHosts(sql: SQL): Promise<void> {
    const [, host] = createHostFixture(
        expectedExistingScanId,
        expectedHostIp,
        expectedExistingHostId,
    );
    await sql`
        INSERT into recon.scan_hosts
            (id, scan_id, ip)
        VALUES
            (${host.id}, ${host.scan_id}, ${host.ip});
    `;
}

async function seedPorts(sql: SQL): Promise<void> {
    const [, port] = createPortFixture(
        expectedExistingHostId,
        expectedPort,
        expectedExistingPortId,
    );
    await sql`
        INSERT into recon.scan_ports
            (id, host_id, port)
        VALUES
            (${port.id}, ${port.host_id}, ${port.port});
    `;
}

export async function seedDb(sql: SQL): Promise<void> {
    await seedUsers(sql);
    await seedProjects(sql);
    await seedScans(sql);
    await seedHosts(sql);
    await seedPorts(sql);
}
