import { sql, type TransactionSQL } from 'bun';
import { readdir } from 'node:fs/promises';

const MAX_RETRIES = 30;
const SLEEP_TIMEOUT = 1000;
const MIGRATIONS_DIR = './migrations/';
const MIGRATION_ADVISORY_LOCK_ID = 1337;
const MIGRATION_TABLE = 'recon.schema_migrations';

type Migration = {
    version: string,
    applied_at: Date,
};

async function acquireAdvisoryLock(trx: TransactionSQL): Promise<void> {
    return trx`SELECT pg_advisory_lock(${MIGRATION_ADVISORY_LOCK_ID})`;
}

async function releaseLock(trx: TransactionSQL): Promise<void> {
    return trx`SELECT pg_advisory_unlock(${MIGRATION_ADVISORY_LOCK_ID})`;
}

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
    throw new Error('DB is unavailable');
}

async function readAplliedMigrations(trx: TransactionSQL): Promise<Array<Migration>> {
    const migrationTable = await trx`SELECT to_regclass(${MIGRATION_TABLE}) as exists`;
    if (migrationTable[0].exists) {
        return trx`SELECT version, applied_at from ${sql(MIGRATION_TABLE)}`;
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

async function applyMigration(trx: TransactionSQL, filename: string): Promise<void> {
    console.info(`Applying ${filename}`);
    await trx.savepoint(async (sp) => {
        await sp.file(`${MIGRATIONS_DIR}/${filename}`);
        console.log('migration applied')

        await sp`INSERT INTO ${sql(MIGRATION_TABLE)} (version) VALUES (${filename})`;
    })
}


async function execMigration() {
    await waitDb();
    await sql.begin(async (trx) => {
        await acquireAdvisoryLock(trx);
        try {
            const migrations: Array<Migration> = await readAplliedMigrations(trx);
            const files: Array<string> = await scanMigrationsDirectory();
            const notApplied: Array<string> = filterMigrations(files, migrations);
            for (const file of notApplied) {
                await applyMigration(trx, file);
            }
        } finally {
            await releaseLock(trx);
        }
    })
}

execMigration();
