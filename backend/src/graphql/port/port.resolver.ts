import type { HostDto } from '@/src/transport/host.dto';
import type { PaginatonResolver } from '../types';
import type { PortService } from './port.service';
import type { PortDto } from '@/src/transport/port.dto';

export type PortResolverFactoryDeps = {
    portService: PortService;
};

export type PortResolver = {
    Host: {
        ports: PaginatonResolver<PortDto, HostDto>;
    };
};

export function createPortResolver({ portService }: PortResolverFactoryDeps): PortResolver {
    return {
        Host: {
            ports: portService.listPorts,
        },
    };
}
