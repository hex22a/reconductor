import type { HostEntity, HostInsertSeed } from '@/src/domain/host.entity';
import { randomUUIDv7 } from 'bun';

export function createHostFixture(
    scanId: string,
    ip: string,
    hostId?: string,
): [HostEntity, HostInsertSeed] {
    const id = hostId || randomUUIDv7();
    const date = new Date();
    return [
        {
            id,
            scan_id: scanId,
            created_at: date,
            ip,
        },
        {
            id,
            scan_id: scanId,
            ip,
        },
    ];
}
