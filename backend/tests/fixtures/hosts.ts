import type { HostEntity, HostInsertSeed } from '@/src/domain/host.entity';
import { randomUUIDv7 } from 'bun';

export function createHostFixture(
    scanRunId: string,
    ip: string,
    hostId?: string,
): [HostEntity, HostInsertSeed] {
    const id = hostId || randomUUIDv7();
    return [
        {
            id,
            scan_run_id: scanRunId,
            ip,
        },
        {
            id,
            scan_run_id: scanRunId,
            ip,
        },
    ];
}
