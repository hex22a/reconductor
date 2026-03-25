import { PortNotFoundError } from '../domain/errors/PortNotFoundError';
import type { PortEntity } from '../domain/port.entity';
import type { SQL } from './db';

export type PortRepositoryDeps = {
    sql: SQL;
};

export type PortRepository = {
    getPort: (id: string) => Promise<PortEntity>;
};

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
    };
}
