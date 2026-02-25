import type { SQL } from 'bun';

const MAX_RETRIES = 30;
const SLEEP_TIMEOUT = 1000;

export async function waitDb(sql: SQL) {
    for (let i = 0; i < MAX_RETRIES; i++) {
        try {
            console.info(`Connecting to DB. Try: ${i}`);
            await sql`SELECT 1`;
            console.info('Connected to DB');
            return;
        } catch {
            console.info('DB not ready. Retrying...');
            await new Promise((r) => setTimeout(r, SLEEP_TIMEOUT));
        }
    }
    throw new Error('DB is unavailable');
}
