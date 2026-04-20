import type { ScanRunRepository } from '@/src/persistence/scanRun';
import type { EntityResolver, PaginatonResolver } from '../types';
import type { ScanRunDto } from '@/src/transport/scanRun.dto';
import type { ScanDto } from '@/src/transport/scan.dto';
import type { CursorDecoder, CursorEncoder } from '@/src/utils/cursor';
import type { Edge } from '@/src/transport/edge.dto';
import type { Pagination } from '@/src/transport/pagination.dto';

export type ScanRunServiceFactoryDeps = {
    scanRunRepository: ScanRunRepository;
    encodeCursor: CursorEncoder;
    decodeCursor: CursorDecoder;
};

export type GetScanRunArgs = {
    id: string;
};

export type ListScanRunsArgs = {
    first: number;
    after: string;
};

export type ScanRunService = {
    getScanRun: EntityResolver<ScanRunDto, GetScanRunArgs>;
    listScanRuns: PaginatonResolver<ScanRunDto, ScanDto>;
};

export function createScanRunService({
    scanRunRepository,
    encodeCursor,
}: ScanRunServiceFactoryDeps): ScanRunService {
    return {
        async getScanRun(_, { id }: GetScanRunArgs): Promise<ScanRunDto> {
            const scanRun = await scanRunRepository.getScanRun(id);
            return {
                id: scanRun.id,
                scan_id: scanRun.scan_id,
                created_at: scanRun.created_at,
            };
        },
        async listScanRuns(parent: ScanDto): Promise<Pagination<Edge<ScanRunDto>>> {
            const { scanRuns, hasNextPage } = await scanRunRepository.listScanRuns(parent.id);
            const edges = scanRuns.map((scanRunEntity) => ({
                node: {
                    id: scanRunEntity.id,
                    scan_id: scanRunEntity.scan_id,
                    created_at: scanRunEntity.created_at,
                },
                cursor: encodeCursor(scanRunEntity.id),
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
