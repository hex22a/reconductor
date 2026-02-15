import { sql, type TransactionSQL } from '@/src/persistence/db';

const ROLLBACK = Symbol('TEST_ROLLBACK');

export async function withTrx(
    testFn: (trx: TransactionSQL) => Promise<void>
): Promise<void> {
    await sql.begin(async (trx: TransactionSQL) => {
        try {
            await testFn(trx);
            throw ROLLBACK
        } catch (error) {
            if (error !== ROLLBACK) {
                throw error;
            }
        }
    });
}
