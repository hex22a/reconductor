import type { HostDto } from '@/src/transport/host.dto';
import type { EntityResolver, PaginatonResolver } from '../types';
import type { GetHostArgs, HostService } from './host.service';
import type { ScanRunDto } from '@/src/transport/scanRun.dto';

export type HostResolverFactoryDeps = {
    hostService: HostService;
};

export type HostResolver = {
    Query: {
        host: EntityResolver<HostDto, GetHostArgs>;
    };
    ScanRun: {
        hosts: PaginatonResolver<HostDto, ScanRunDto>;
    };
};

export function createHostResolver({ hostService }: HostResolverFactoryDeps): HostResolver {
    return {
        Query: {
            host: hostService.getHost,
        },
        ScanRun: {
            hosts: hostService.listHosts,
        },
    };
}
