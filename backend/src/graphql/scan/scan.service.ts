import type { ScanRepository } from '@/src/persistence/scan.db';
import type { CursorDecoder, CursorEncoder } from '@/src/utils/cursor';
import type { MutationResolver, PaginatonResolver } from '../types';
import type { CreateScanDto, ScanDto } from '@/src/transport/scan.dto';
import type { Pagination } from '@/src/transport/pagination.dto';
import type { Edge } from '@/src/transport/edge.dto';
import type { CreateEntityPayload } from '@/src/transport/payload.dto';
import type { ScanEntity } from '@/src/domain/scan.entity';
import { scanSchema } from '@/src/transport/scan.schema';

export type CreateScanArgs = {
    input: CreateScanDto;
};

export type ListScansArgs = {
    projectId: string;
};

export type ScanServiceFactoryDeps = {
    scanRepository: ScanRepository;
    encodeCursor: CursorEncoder;
    decodeCursor: CursorDecoder;
};

export type ScanService = {
    listScans: PaginatonResolver<ScanDto, ListScansArgs>;
    createScan: MutationResolver<ScanDto, CreateScanArgs>;
};

export function createScanService({
    scanRepository,
    encodeCursor,
}: ScanServiceFactoryDeps): ScanService {
    return {
        async listScans(_: unknown, args: ListScansArgs): Promise<Pagination<Edge<ScanDto>>> {
            const { scans, hasNextPage } = await scanRepository.listScans(args.projectId);
            const edges = scans.map((scanEntity) => ({
                node: {
                    id: scanEntity.id,
                    created_at: scanEntity.created_at,
                    target: scanEntity.target,
                    status: scanEntity.status,
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
        async createScan(
            _,
            { input }: CreateScanArgs,
        ): Promise<CreateEntityPayload<Edge<ScanDto>>> {
            const { target, projectId, schedule }: CreateScanDto = scanSchema.parse(input);
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
                        status: scan.status,
                    },
                    cursor: encodeCursor(scan.id),
                },
                errors: [],
            };
        },
    };
}
