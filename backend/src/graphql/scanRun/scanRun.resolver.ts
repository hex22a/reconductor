import type { ScanRunDto } from '@/src/transport/scanRun.dto';
import type { EntityResolver, PaginatonResolver } from '../types';
import type { GetScanRunArgs, ScanRunService } from './scanRun.service';
import type { ScanDto } from '@/src/transport/scan.dto';

export type ScanRunResolverFactoryDeps = {
    scanRunService: ScanRunService;
};

export type ScanRunResolver = {
    Query: {
        run: EntityResolver<ScanRunDto, GetScanRunArgs>;
    };
    Scan: {
        runs: PaginatonResolver<ScanRunDto, ScanDto>;
    };
};

export function createScanRunResolver({
    scanRunService,
}: ScanRunResolverFactoryDeps): ScanRunResolver {
    return {
        Query: {
            run: scanRunService.getScanRun,
        },
        Scan: {
            runs: scanRunService.listScanRuns,
        },
    };
}
