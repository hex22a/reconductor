import { afterEach, describe, expect, mock, test } from 'bun:test';
import {
    createScanResolver,
    type ScanResolver,
    type ScanResolverFactoryDeps,
} from './scan.resolver';
import type { ScanService } from './scan.service';

describe('scan.resolver', () => {
    const mockCreateScan = mock();
    const mockListScans = mock();
    const mockScanService: ScanService = {
        createScan: mockCreateScan,
        listScans: mockListScans,
    };
    const expectedScanResolverFactoryDeps: ScanResolverFactoryDeps = {
        scanService: mockScanService,
    };

    afterEach(() => {
        mockCreateScan.mockReset();
        mockListScans.mockReset();
    });

    test('createScanResolver', () => {
        // Arrange
        const expectedScanResolver: ScanResolver = {
            Query: {
                scans: mockListScans,
            },
            Mutation: {
                createScan: mockCreateScan,
            },
        };
        // Act
        const actualScanResolver: ScanResolver = createScanResolver(
            expectedScanResolverFactoryDeps,
        );
        // Assert
        expect(actualScanResolver).toEqual(expectedScanResolver);
    });
});
