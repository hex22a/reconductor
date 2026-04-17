import { SCAN_RUNS_PAGE_SIZE } from '../constants';
import { ScanRunNotFoundError } from '../domain/errors/ScanRunNotFoundError';
import type { ScanRunEntity } from '../domain/scanRun.entity';
import type { SQL } from './db';

export type ScanRunRepositoryDeps = {
    sql: SQL;
};

export interface ScanRunRepository {
    getScanRun(id: string): Promise<ScanRunEntity>;
    listScanRuns(scanId: string): Promise<{ scanRuns: Array<ScanRunEntity>; hasNextPage: boolean }>;
    listScanRuns(
        scanId: string,
        curstor: string,
    ): Promise<{ scanRuns: Array<ScanRunEntity>; hasNextPage: boolean }>;
}

export function createScanRunRepository({ sql }: ScanRunRepositoryDeps): ScanRunRepository {
    return {
        async getScanRun(id: string): Promise<ScanRunEntity> {
            const [scanRun] = await sql<Array<ScanRunEntity>>`
                SELECT
                    id,
                    scan_id,
                    created_at
                FROM recon.scan_runs
                WHERE id=${id}
                LIMIT 1;
            `;
            if (!scanRun) {
                throw new ScanRunNotFoundError();
            }
            return scanRun;
        },
        async listScanRuns(
            scanId: string,
            cursor?: string,
        ): Promise<{ scanRuns: Array<ScanRunEntity>; hasNextPage: boolean }> {
            const limit = SCAN_RUNS_PAGE_SIZE + 1;
            const scanRuns: Array<ScanRunEntity> = await sql`
                SELECT
                    id,
                    scan_id,
                    created_at
                FROM recon.scan_runs
                WHERE scan_id=${scanId}
                ${cursor ? sql`AND id < ${cursor}` : sql``}
                ORDER BY id DESC
                LIMIT ${limit};
            `;
            const hasNextPage = scanRuns.length === limit;
            return {
                scanRuns: hasNextPage ? scanRuns.slice(0, -1) : scanRuns,
                hasNextPage,
            };
        },
    };
}
