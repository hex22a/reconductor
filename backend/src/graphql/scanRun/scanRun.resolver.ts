import type { ScanRunDto } from '@/src/transport/scanRun.dto';
import type { PaginatonResolver } from '../types';
import type { ScanRunService } from './scanRun.service';
import type { ScanDto } from '@/src/transport/scan.dto';

export type ScanRunResolverFactoryDeps = {
    scanRunService: ScanRunService;
};

export type ScanRunResolver = {
    Scan: {
        runs: PaginatonResolver<ScanRunDto, ScanDto>;
    };
};

export function createScanRunResolver({
    scanRunService,
}: ScanRunResolverFactoryDeps): ScanRunResolver {
    return {
        Scan: {
            runs: scanRunService.listScanRuns,
        },
    };
}
