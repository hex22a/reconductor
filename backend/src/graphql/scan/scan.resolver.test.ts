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
    const mockWithValidation = mock();
    const mockScanService: ScanService = {
        createScan: mockCreateScan,
        listScans: mockListScans,
    };
    const expectedScanResolverFactoryDeps: ScanResolverFactoryDeps = {
        scanService: mockScanService,
        withValidation: mockWithValidation,
    };

    afterEach(() => {
        mockCreateScan.mockReset();
        mockListScans.mockReset();
        mockWithValidation.mockReset();
    });

    test('createScanResolver', () => {
        // Arrange
        const mockDecoratedMutation = mock();
        mockWithValidation.mockReturnValue(mockDecoratedMutation);
        const expectedScanResolver: ScanResolver = {
            Project: {
                scans: mockListScans,
            },
            Mutation: {
                createScan: mockDecoratedMutation,
            },
        };
        // Act
        const actualScanResolver: ScanResolver = createScanResolver(
            expectedScanResolverFactoryDeps,
        );
        // Assert
        expect(actualScanResolver).toEqual(expectedScanResolver);
        expect(mockWithValidation).toHaveBeenCalledWith(mockCreateScan);
    });
});
