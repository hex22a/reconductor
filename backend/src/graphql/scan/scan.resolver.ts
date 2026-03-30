import type { ScanDto } from '@/src/transport/scan.dto';
import type { MutationResolver, PaginatonResolver } from '../types';
import type { CreateScanArgs, ListScansArgs, ScanService } from './scan.service';
import type { WithValidationDecorator } from '../decorators/mutation';

export type ScanResolverFactoryDeps = {
    scanService: ScanService;
    withValidation: WithValidationDecorator;
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
    scanService,
    withValidation,
}: ScanResolverFactoryDeps): ScanResolver {
    return {
        Query: {
            scans: scanService.listScans,
        },
        Mutation: {
            createScan: withValidation(scanService.createScan),
        },
    };
}
