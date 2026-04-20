import type { ScanRepository } from '@/src/persistence/scan.db';
import type { CursorDecoder, CursorEncoder } from '@/src/utils/cursor';
import type { EntityResolver, MutationResolver, PaginatonResolver } from '../types';
import type { CreateScanDto, ScanDto } from '@/src/transport/scan.dto';
import type { Pagination } from '@/src/transport/pagination.dto';
import type { Edge } from '@/src/transport/edge.dto';
import type { CreateEntityPayload } from '@/src/transport/payload.dto';
import type { ScanEntity } from '@/src/domain/scan.entity';
import { scanSchema } from '@/src/transport/scan.schema';
import type { ProjectDto } from '@/src/transport/project.dto';
import type { QueueService } from '@/src/queue/queue.service';
import type { CronParser } from '@/src/utils/cron';

export type CreateScanArgs = {
    input: CreateScanDto;
};

export type GetScanArgs = {
    id: string;
};

export type ListScansArgs = {
    first: number;
    after: string;
};

export type ScanServiceFactoryDeps = {
    scanRepository: ScanRepository;
    queueService: QueueService;
    encodeCursor: CursorEncoder;
    decodeCursor: CursorDecoder;
    cronParser: CronParser;
};

export type ScanService = {
    getScan: EntityResolver<ScanDto, GetScanArgs>;
    listScans: PaginatonResolver<ScanDto, ProjectDto>;
    createScan: MutationResolver<ScanDto, CreateScanArgs>;
};

export function createScanService({
    scanRepository,
    queueService,
    encodeCursor,
    cronParser,
}: ScanServiceFactoryDeps): ScanService {
    return {
        async getScan(_, { id }: GetScanArgs): Promise<ScanDto> {
            const scan: ScanEntity = await scanRepository.getScan(id);
            return {
                id: scan.id,
                target: scan.target,
                status: scan.status,
                schedule: scan.schedule,
                created_at: scan.created_at,
            };
        },
        async listScans(parent: ProjectDto): Promise<Pagination<Edge<ScanDto>>> {
            const { scans, hasNextPage } = await scanRepository.listScans(parent.id);
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
            let nextRunAt;
            if (schedule) {
                nextRunAt = cronParser.getNextRunDate(schedule);
            }
            const scan: ScanEntity = await scanRepository.createScan({
                target,
                project_id: projectId,
                next_run_at: nextRunAt,
                schedule: schedule,
            });
            await queueService.publish({
                id: scan.id,
                target,
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
