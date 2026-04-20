import { afterEach, describe, expect, mock, test } from 'bun:test';
import type { ScanRunService } from './scanRun.service';
import {
    createScanRunResolver,
    type ScanRunResolver,
    type ScanRunResolverFactoryDeps,
} from './scanRun.resolver';

describe('scanRun.resolver', () => {
    const mockListScanRuns = mock();
    const mockGetScanRuns = mock();
    const mockScanRunService: ScanRunService = {
        getScanRun: mockGetScanRuns,
        listScanRuns: mockListScanRuns,
    };
    const expectedScanResolverFactoryDeps: ScanRunResolverFactoryDeps = {
        scanRunService: mockScanRunService,
    };

    afterEach(() => {
        mockGetScanRuns.mockReset();
        mockListScanRuns.mockReset();
    });

    test('createScanRunResolver', () => {
        // Arrange
        const expectedScanRunResolver: ScanRunResolver = {
            Query: {
                run: mockGetScanRuns,
            },
            Scan: {
                runs: mockListScanRuns,
            },
        };
        // Act
        const actualScanRunResolver: ScanRunResolver = createScanRunResolver(
            expectedScanResolverFactoryDeps,
        );
        // Assert
        expect(actualScanRunResolver).toEqual(expectedScanRunResolver);
    });
});
