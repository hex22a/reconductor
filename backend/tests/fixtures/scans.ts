import type { ScanEntity, ScanInsert, ScanInsertSeed } from '@/src/domain/scan.entity';
import { randomUUIDv7 } from 'bun';

export const expectedExistingScanId = '019c8a62-739f-73f1-99a8-e94a34bc31f3';
export const expectedScanTarget = '192.168.1.0/24';
export const expectedNextRunAt = new Date(2030, 6, 15, 1, 30, 30, 10);

export function createScanFixture(
    projectId: string,
    target: string,
    nextRunAt: Date,
    schedule?: string,
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
            next_run_at: nextRunAt,
            status: 'scheduled',
            schedule: schedule ?? null,
        },
        {
            project_id: projectId,
            target,
            next_run_at: nextRunAt,
            schedule: schedule ?? null,
        },
        {
            id,
            project_id: projectId,
            target,
            next_run_at: nextRunAt,
            schedule: schedule ?? null,
        },
    ];
}
