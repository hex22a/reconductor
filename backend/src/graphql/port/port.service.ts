import type { CursorDecoder, CursorEncoder } from '@/src/utils/cursor';
import type { PaginatonResolver } from '../types';
import type { HostDto } from '@/src/transport/host.dto';
import type { Pagination } from '@/src/transport/pagination.dto';
import type { Edge } from '@/src/transport/edge.dto';
import type { PortRepository } from '@/src/persistence/port.db';
import type { PortDto } from '@/src/transport/port.dto';

export type PortServiceFactoryDeps = {
    portRepository: PortRepository;
    encodeCursor: CursorEncoder;
    decodeCursor: CursorDecoder;
};

export type ListPortsArgs = {
    first: number;
    after: string;
};

export type PortService = {
    listPorts: PaginatonResolver<PortDto, HostDto>;
};

export function createPortService({
    portRepository,
    encodeCursor,
}: PortServiceFactoryDeps): PortService {
    return {
        async listPorts(parent: HostDto): Promise<Pagination<Edge<PortDto>>> {
            const { ports, hasNextPage } = await portRepository.listPorts(parent.id);
            const edges = ports.map((portEntity) => ({
                node: {
                    id: portEntity.id,
                    port: portEntity.port,
                    protocol: portEntity.product,
                    state: portEntity.state,
                    product: portEntity.product,
                    version: portEntity.version,
                },
                cursor: encodeCursor(portEntity.id),
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
