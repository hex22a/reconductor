import { PORTS_PAGE_SIZE } from '../constants';
import { PortNotFoundError } from '../domain/errors/PortNotFoundError';
import type { PortEntity } from '../domain/port.entity';
import type { SQL } from './db';

export type PortRepositoryDeps = {
    sql: SQL;
};

export interface PortRepository {
    getPort(id: string): Promise<PortEntity>;
    listPorts(hostId: string): Promise<{ ports: Array<PortEntity>; hasNextPage: boolean }>;
    listPorts(
        hostId: string,
        cursor: string,
    ): Promise<{ ports: Array<PortEntity>; hasNextPage: boolean }>;
}

export function createPortRepository({ sql }: PortRepositoryDeps): PortRepository {
    return {
        async getPort(id: string): Promise<PortEntity> {
            const [port] = await sql<Array<PortEntity>>`
                SELECT
                    id,
                    host_id,
                    port,
                    protocol,
                    state,
                    service,
                    product,
                    version
                FROM recon.scan_ports
                WHERE id=${id}
                LIMIT 1;
            `;
            if (!port) {
                throw new PortNotFoundError();
            }
            return port;
        },
        async listPorts(
            hostId: string,
            cursor?: string,
        ): Promise<{ ports: Array<PortEntity>; hasNextPage: boolean }> {
            const limit = PORTS_PAGE_SIZE + 1;
            const ports: Array<PortEntity> = await sql`
                SELECT
                    id,
                    host_id,
                    port,
                    protocol,
                    state,
                    service,
                    product,
                    version
                FROM recon.scan_ports
                WHERE host_id=${hostId}
                ${cursor ? sql`AND id < ${cursor}` : sql``}
                LIMIT ${limit};
            `;
            const hasNextPage = ports.length === limit;
            return {
                ports: hasNextPage ? ports.slice(0, -1) : ports,
                hasNextPage,
            };
        },
    };
}
