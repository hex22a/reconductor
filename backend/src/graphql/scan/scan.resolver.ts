import type { ScanDto } from '@/src/transport/scan.dto';
import type { MutationResolver, PaginatonResolver } from '../types';
import type { CreateScanArgs, ListScansArgs, ScanService } from './scan.service';

export type ScanResolverFactoryDeps = {
    scanService: ScanService;
};

export type ScanResolver = {
    Query: {
        scans: PaginatonResolver<ScanDto, ListScansArgs>;
    };
    Mutation: {
        createScan: MutationResolver<ScanDto, CreateScanArgs>;
    };
};

export function createScanResolver({ scanService }: ScanResolverFactoryDeps): ScanResolver {
    return {
        Query: {
            scans: scanService.listScans,
        },
        Mutation: {
            createScan: scanService.createScan,
        },
    };
}
