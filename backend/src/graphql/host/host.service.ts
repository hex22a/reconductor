import type { HostRepository } from '@/src/persistence/host.db';
import type { CursorDecoder, CursorEncoder } from '@/src/utils/cursor';
import type { EntityResolver, PaginatonResolver } from '../types';
import type { HostDto } from '@/src/transport/host.dto';
import type { ScanRunDto } from '@/src/transport/scanRun.dto';
import type { Pagination } from '@/src/transport/pagination.dto';
import type { Edge } from '@/src/transport/edge.dto';
import type { HostEntity } from '@/src/domain/host.entity';

export type HostServiceFactoryDeps = {
    hostRepository: HostRepository;
    encodeCursor: CursorEncoder;
    decodeCursor: CursorDecoder;
};

export type GetHostArgs = {
    id: string;
};

export type ListHostsArgs = {
    first: number;
    after: string;
};

export type HostService = {
    getHost: EntityResolver<HostDto, GetHostArgs>;
    listHosts: PaginatonResolver<HostDto, ScanRunDto>;
};

export function createHostService({
    hostRepository,
    encodeCursor,
}: HostServiceFactoryDeps): HostService {
    return {
        async getHost(_, { id }: GetHostArgs): Promise<HostDto> {
            const host: HostEntity = await hostRepository.getHost(id);
            return {
                id: host.id,
                ip: host.ip,
                hostname: host.hostname,
                os_match: host.os_match,
                os_accuracy: host.os_accuracy,
                vendor: host.vendor,
                mac: host.mac,
            };
        },
        async listHosts(parent: ScanRunDto): Promise<Pagination<Edge<HostDto>>> {
            const { hosts, hasNextPage } = await hostRepository.listHosts(parent.id);
            const edges = hosts.map((hostEntity) => ({
                node: {
                    id: hostEntity.id,
                    ip: hostEntity.ip,
                    mac: hostEntity.mac,
                    hostname: hostEntity.hostname,
                    vendor: hostEntity.vendor,
                    os_match: hostEntity.os_match,
                    os_accuracy: hostEntity.os_accuracy,
                },
                cursor: encodeCursor(hostEntity.id),
            }));
            return {
                edges,
                pageInfo: {
                    hasNextPage,
                    hasPreviousPage: false,
                    startCursor: edges.at(0)?.cursor ?? null,
                    endCursor: edges.at(-1)?.cursor ?? null,
                },
            };
        },
    };
}
