import type { PortEntity, PortInsertSeed } from '@/src/domain/port.entity';
import { randomUUIDv7 } from 'bun';

export function createPortFixture(
    hostId: string,
    port: number,
    portId?: string,
): [PortEntity, PortInsertSeed] {
    const id = portId || randomUUIDv7();
    return [
        {
            id,
            host_id: hostId,
            port,
        },
        {
            id,
            host_id: hostId,
            port,
        },
    ];
}
