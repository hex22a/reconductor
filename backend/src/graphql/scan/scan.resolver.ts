import type { ScanDto } from '@/src/transport/scan.dto';
import type { EntityResolver, MutationResolver, PaginatonResolver } from '../types';
import type { CreateScanArgs, GetScanArgs, ScanService } from './scan.service';
import type { WithValidationDecorator } from '../decorators/mutation';
import type { ProjectDto } from '@/src/transport/project.dto';

export type ScanResolverFactoryDeps = {
    scanService: ScanService;
    withValidation: WithValidationDecorator;
};

export type ScanResolver = {
    Query: {
        scan: EntityResolver<ScanDto, GetScanArgs>;
    };
    Mutation: {
        createScan: MutationResolver<ScanDto, CreateScanArgs>;
    };
    Project: {
        scans: PaginatonResolver<ScanDto, ProjectDto>;
    };
};

export function createScanResolver({
    scanService,
    withValidation,
}: ScanResolverFactoryDeps): ScanResolver {
    return {
        Query: {
            scan: scanService.getScan,
        },
        Mutation: {
            createScan: withValidation(scanService.createScan),
        },
        Project: {
            scans: scanService.listScans,
        },
    };
}
