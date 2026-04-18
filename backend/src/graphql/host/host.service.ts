import type { HostRepository } from '@/src/persistence/host.db';
import type { CursorDecoder, CursorEncoder } from '@/src/utils/cursor';
import type { PaginatonResolver } from '../types';
import type { HostDto } from '@/src/transport/host.dto';
import type { ScanRunDto } from '@/src/transport/scanRun.dto';
import type { Pagination } from '@/src/transport/pagination.dto';
import type { Edge } from '@/src/transport/edge.dto';

export type HostServiceFactoryDeps = {
    hostRepository: HostRepository;
    encodeCursor: CursorEncoder;
    decodeCursor: CursorDecoder;
};

export type ListHostsArgs = {
    first: number;
    after: string;
};

export type HostService = {
    listHosts: PaginatonResolver<HostDto, ScanRunDto>;
};

export function createHostService({
    hostRepository,
    encodeCursor,
}: HostServiceFactoryDeps): HostService {
    return {
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
