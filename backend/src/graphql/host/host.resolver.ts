import type { HostDto } from '@/src/transport/host.dto';
import type { PaginatonResolver } from '../types';
import type { HostService } from './host.service';
import type { ScanRunDto } from '@/src/transport/scanRun.dto';

export type HostResolverFactoryDeps = {
    hostService: HostService;
};

export type HostResolver = {
    ScanRun: {
        hosts: PaginatonResolver<HostDto, ScanRunDto>;
    };
};

export function createHostResolver({ hostService }: HostResolverFactoryDeps): HostResolver {
    return {
        ScanRun: {
            hosts: hostService.listHosts,
        },
    };
}
