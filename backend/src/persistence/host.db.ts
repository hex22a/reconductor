import { HostNotFoundError } from '../domain/errors/HostNotFoundError';
import type { HostEntity } from '../domain/host.entity';
import type { SQL } from './db';

export type HostRepositoryDeps = {
    sql: SQL;
};

export type HostRepository = {
    getHost: (id: string) => Promise<HostEntity>;
};

export function createHostRepository({ sql }: HostRepositoryDeps): HostRepository {
    return {
        async getHost(id: string): Promise<HostEntity> {
            const [host] = await sql<Array<HostEntity>>`
                SELECT
                    id,
                    scan_run_id,
                    ip,
                    mac,
                    vendor,
                    hostname,
                    os_match,
                    os_accuracy
                FROM recon.scan_hosts
                WHERE id=${id}
                LIMIT 1;
            `;
            if (!host) {
                throw new HostNotFoundError();
            }
            return host;
        },
    };
}
