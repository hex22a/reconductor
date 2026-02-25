import type { ScanEntity, ScanInsert, ScanInsertSeed } from '@/src/domain/scan.entity';
import { randomUUIDv7 } from 'bun';

export const expectedExistingScanId = '019c8a62-739f-73f1-99a8-e94a34bc31f3';
export const expectedScanTarget = '192.168.1.0/24';

export function createScanFixture(
    projectId: string,
    target: string,
    scanId?: string,
): [ScanEntity, ScanInsert, ScanInsertSeed] {
    const id = scanId || randomUUIDv7();
    const date = new Date();
    return [
        {
            id,
            project_id: projectId,
            target,
            created_at: date,
            status: 'scheduled',
        },
        {
            project_id: projectId,
            target,
        },
        {
            id,
            project_id: projectId,
            target,
        },
    ];
}
