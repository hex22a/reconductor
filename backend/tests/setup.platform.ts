import { beforeAll } from 'bun:test';
import { sql } from '@/src/persistence/db';
import { seedDb } from '@/db/seed';

beforeAll(async () => {
    await seedDb(sql);
});
