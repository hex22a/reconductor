import { ScanNotFoundError } from "../domain/errors/ScanNotFoundError";
import type { ScanEntity, ScanInsert } from "../domain/scan.entity";
import type { SQL } from "./db";

export type ScanRepositoryDeps = {
    sql: SQL,
};

export type ScanRepository = {
    createScan: (scan: ScanInsert) => Promise<ScanEntity>;
    getScan: (id: string) => Promise<ScanEntity>;
};

export function createScanRepository({ sql }: ScanRepositoryDeps): ScanRepository {
    return {
        async createScan(scan: ScanInsert): Promise<ScanEntity> {
            const queryResult = await sql`
                INSERT INTO recon.scans
                    (project_id, target)
                VALUES
                    (${scan.project_id}, ${scan.target})
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
                    status
                FROM recon.scans
                WHERE id=${id}
                LIMIT 1;
            `;
            if (!scan) {
                throw new ScanNotFoundError();
            }
            return scan;
        }
    }
}
