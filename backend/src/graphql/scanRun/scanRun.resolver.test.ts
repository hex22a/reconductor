import { afterEach, describe, expect, mock, test } from 'bun:test';
import type { ScanRunService } from './scanRun.service';
import {
    createScanRunResolver,
    type ScanRunResolver,
    type ScanRunResolverFactoryDeps,
} from './scanRun.resolver';

describe('scanRun.resolver', () => {
    const mockListScanRuns = mock();
    const mockScanRunService: ScanRunService = {
        listScanRuns: mockListScanRuns,
    };
    const expectedScanResolverFactoryDeps: ScanRunResolverFactoryDeps = {
        scanRunService: mockScanRunService,
    };

    afterEach(() => {
        mockListScanRuns.mockReset();
    });

    test('createScanRunResolver', () => {
        // Arrange
        const expectedScanResolver: ScanRunResolver = {
            Scan: {
                scan_runs: mockListScanRuns,
            },
        };
        // Act
        const actualScanResolver: ScanRunResolver = createScanRunResolver(
            expectedScanResolverFactoryDeps,
        );
        // Assert
        expect(actualScanResolver).toEqual(expectedScanResolver);
    });
});
