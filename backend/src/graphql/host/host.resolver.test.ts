import { afterEach, describe, expect, mock, test } from 'bun:test';
import type { HostService } from './host.service';
import {
    createHostResolver,
    type HostResolver,
    type HostResolverFactoryDeps,
} from './host.resolver';

describe('host.resolver', () => {
    const mockListHosts = mock();
    const mockHostService: HostService = {
        listHosts: mockListHosts,
    };
    const expectedHostResolverFactoryDeps: HostResolverFactoryDeps = {
        hostService: mockHostService,
    };

    afterEach(() => {
        mockListHosts.mockReset();
    });

    test('createScanRunResolver', () => {
        // Arrange
        const expectedScanResolver: HostResolver = {
            ScanRun: {
                hosts: mockListHosts,
            },
        };
        // Act
        const actualScanResolver: HostResolver = createHostResolver(
            expectedHostResolverFactoryDeps,
        );
        // Assert
        expect(actualScanResolver).toEqual(expectedScanResolver);
    });
});
