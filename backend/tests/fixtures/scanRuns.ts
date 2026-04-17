import type { ScanRunEntity } from '@/src/domain/scanRun.entity';
import { randomUUIDv7 } from 'bun';

export const expectedExistingScanRunId = '019d9bb1-f513-71c6-a359-a7c8f837ad35';

export function createScanRunFixture(scan_id: string, scan_run_id?: string): ScanRunEntity {
    const id = scan_run_id || randomUUIDv7();
    const date = new Date();
    return {
        id,
        scan_id,
        created_at: date,
    };
}
