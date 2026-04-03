import { SCANS_PAGE_SIZE } from '../constants';
import { ScanNotFoundError } from '../domain/errors/ScanNotFoundError';
import type { ScanEntity, ScanInsert } from '../domain/scan.entity';
import type { SQL } from './db';

export type ScanRepositoryDeps = {
    sql: SQL;
};

export interface ScanRepository {
    createScan(scan: ScanInsert): Promise<ScanEntity>;
    getScan(id: string): Promise<ScanEntity>;
    listScans(projectId: string): Promise<{ scans: Array<ScanEntity>; hasNextPage: boolean }>;
    listScans(
        projectId: string,
        cursor: string,
    ): Promise<{ scans: Array<ScanEntity>; hasNextPage: boolean }>;
}

export function createScanRepository({ sql }: ScanRepositoryDeps): ScanRepository {
    return {
        async createScan(scan: ScanInsert): Promise<ScanEntity> {
            const queryResult = await sql`
                INSERT INTO recon.scans
                    (project_id, target, schedule, next_run_at)
                VALUES
                    (${scan.project_id}, ${scan.target}, ${scan.schedule}, ${scan.next_run_at})
                RETURNING *;
            `;
            return queryResult[0];
        },
        async getScan(id: string): Promise<ScanEntity> {
            const [scan] = await sql<Array<ScanEntity>>`
                SELECT
                    id,
                    project_id,
                    created_at,
                    target,
                    status,
                    schedule
                FROM recon.scans
                WHERE id=${id}
                LIMIT 1;
            `;
            if (!scan) {
                throw new ScanNotFoundError();
            }
            return scan;
        },
        async listScans(
            projectId: string,
            cursorId?: string,
        ): Promise<{ scans: Array<ScanEntity>; hasNextPage: boolean }> {
            const limit = SCANS_PAGE_SIZE + 1;
            const scans: Array<ScanEntity> = await sql<Array<ScanEntity>>`
                SELECT
                    id,
                    project_id,
                    created_at,
                    target,
                    status,
                    schedule
                FROM recon.scans
                WHERE project_id=${projectId}
                ${cursorId ? sql`AND id < ${cursorId}` : sql``}
                ORDER BY id DESC
                LIMIT ${limit};
            `;
            const hasNextPage = scans.length === limit;
            return {
                scans: hasNextPage ? scans.slice(0, -1) : scans,
                hasNextPage,
            };
        },
    };
}
