import type { ScanEntity } from '@/src/domain/scan.entity';
import type { ScanRepository } from '@/src/persistence/scan.db';
import type { Edge } from '@/src/transport/edge.dto';
import type { Pagination } from '@/src/transport/pagination.dto';
import type { CreateEntityPayload } from '@/src/transport/payload.dto';
import type { ScanDto } from '@/src/transport/scan.dto';
import type { CursorDecoder, CursorEncoder } from '@/src/utils/cursor';
import type { MutationResolver, PaginatonResolver } from './types';

export type CreateScanArgs = {
    input: {
        target: string;
        projectId: string;
        schedule?: string;
    };
};

export type ListScansArgs = {
    projectId: string;
};

export type ScanResolverFactoryDeps = {
    scanRepository: ScanRepository;
    encodeCursor: CursorEncoder;
    decodeCursor: CursorDecoder;
};

export type ScanResolver = {
    Query: {
        scans: PaginatonResolver<ScanDto, ListScansArgs>;
    };
    Mutation: {
        createScan: MutationResolver<ScanDto, CreateScanArgs>;
    };
};

export function createScanResolver({
    scanRepository,
    encodeCursor,
}: ScanResolverFactoryDeps): ScanResolver {
    return {
        Query: {
            async scans(_: unknown, args: ListScansArgs): Promise<Pagination<Edge<ScanDto>>> {
                const { scans, hasNextPage } = await scanRepository.listScans(args.projectId);
                const edges = scans.map((scanEntity) => ({
                    node: {
                        id: scanEntity.id,
                        created_at: scanEntity.created_at,
                        target: scanEntity.target,
                    },
                    cursor: encodeCursor(scanEntity.id),
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
        },
        Mutation: {
            async createScan(
                _,
                { input: { target, projectId, schedule } }: CreateScanArgs,
            ): Promise<CreateEntityPayload<Edge<ScanDto>>> {
                const scan: ScanEntity = await scanRepository.createScan({
                    target,
                    project_id: projectId,
                    schedule: schedule ?? null,
                });
                return {
                    edge: {
                        node: {
                            id: scan.id,
                            created_at: scan.created_at,
                            target,
                        },
                        cursor: encodeCursor(scan.id),
                    },
                    errors: [],
                };
            },
        },
    };
}
