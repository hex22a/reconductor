import { sql, type TransactionSQL } from '@/src/persistence/db';

const ROLLBACK = Symbol('TEST_ROLLBACK');

export async function withTrx(
    testFn: (trx: TransactionSQL) => Promise<void>
): Promise<void> {
    await sql.begin(async (trx: TransactionSQL) => {
        await testFn(trx);
        throw ROLLBACK;
    });
};

export async function catchRollback<T>(
    testFn: () => Promise<T>
): Promise<T | undefined> {
    try {
        return await testFn();
    } catch (error) {
        if (error !== ROLLBACK) {
            throw error;
        }
    }
}
