import { file, sql } from 'bun';
import { readdir } from 'node:fs/promises';

const MAX_RETRIES = 30;
const SLEEP_TIMEOUT = 1000;
const MIGRATIONS_DIR = './migrations/';

type Migration = {
    version: string,
    applied_at: Date,
};

async function waitDb() {
    for (let i = 0; i < MAX_RETRIES; i++) {
        try {
            console.info(`Connecting to DB. Try: ${i}`)
            await sql`SELECT 1`;
            console.info('Connected to DB');
            return;
        } catch {
            console.info('DB not ready. Retrying...');
            await new Promise(r => setTimeout(r, SLEEP_TIMEOUT));
        }
    }
}

async function readAplliedMigrations(): Promise<Array<Migration>> {
    const migrationTable = await sql`SELECT to_regclass('recon.applied_migrations') as exists`;
    if (migrationTable[0].exists) {
        return sql`SELECT version, applied_at from recon.schema_migrations`;
    }
    return [];
}

async function scanMigrationsDirectory(): Promise<Array<string>> {
    return (await readdir(MIGRATIONS_DIR)).sort();
}

function filterMigrations(files: Array<string>, appliedMigrations: Array<Migration>): Array<string> {
    const mappedAppliedMigrations: Array<string> = appliedMigrations.map(am => am.version);
    return files.filter(file => mappedAppliedMigrations.indexOf(file) === -1)
}

async function applyMigration(filename: string): Promise<void> {
    await sql.begin(async (trx) => {
        await trx.file(`${MIGRATIONS_DIR}/${filename}`);
        console.log('migration applied')

        await trx`INSERT INTO recon.schema_migrations (version) VALUES (${filename})`;
    })
}

await waitDb();
const migrations: Array<Migration> = await readAplliedMigrations();
const files: Array<string> = await scanMigrationsDirectory();
const notApplied: Array<string> = filterMigrations(files, migrations);
applyMigration(notApplied[0]!);
console.log(migrations);
console.log(files);
console.log(notApplied);
