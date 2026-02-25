import { sql, type TransactionSQL } from '@/src/persistence/db';
import { readdir } from 'node:fs/promises';
import path from 'node:path';
import { waitDb } from './helpers';

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
    return files.filter(file => mappedAppliedMigrations.indexOf(file) === -1);
}

async function applyMigration(trx: TransactionSQL, filename: string): Promise<void> {
    console.info(`Applying ${filename}`);
    await trx.savepoint(async (sp) => {
        await sp.file(path.resolve(MIGRATIONS_DIR, filename));
        console.log('migration applied');

        await sp`INSERT INTO ${sql(MIGRATION_TABLE)} (version) VALUES (${filename})`;
    });
}


async function execMigration() {
    await waitDb(sql);
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
    });
}

execMigration();
