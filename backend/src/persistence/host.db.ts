import { HOSTS_PAGE_SIZE } from '../constants';
import { HostNotFoundError } from '../domain/errors/HostNotFoundError';
import type { HostEntity } from '../domain/host.entity';
import type { SQL } from './db';

export type HostRepositoryDeps = {
    sql: SQL;
};

export interface HostRepository {
    getHost(id: string): Promise<HostEntity>;
    listHosts(scanRunId: string): Promise<{ hosts: Array<HostEntity>; hasNextPage: boolean }>;
    listHosts(
        scanRunId: string,
        cursor: string,
    ): Promise<{ hosts: Array<HostEntity>; hasNextPage: boolean }>;
}

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
        async listHosts(
            scanRunId: string,
            cursor?: string,
        ): Promise<{ hosts: Array<HostEntity>; hasNextPage: boolean }> {
            const limit = HOSTS_PAGE_SIZE + 1;
            const hosts: Array<HostEntity> = await sql`
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
                WHERE scan_run_id=${scanRunId}
                ${cursor ? sql`AND id < ${cursor}` : sql``}
                ORDER BY id DESC
                LIMIT ${limit};
            `;
            const hasNextPage = hosts.length === limit;
            return {
                hosts: hasNextPage ? hosts.slice(0, -1) : hosts,
                hasNextPage,
            };
        },
    };
}
