import { afterEach, describe, expect, mock, test } from 'bun:test';
import type { GraphQlContext } from '@/src/transport/graphql.context';
import type { PageInfo } from '@/src/transport/pageInfo.dto';
import type { Edge } from '@/src/transport/edge.dto';
import type { Pagination } from '@/src/transport/pagination.dto';
import {
    createScanRunService,
    type GetScanRunArgs,
    type ListScanRunsArgs,
    type ScanRunService,
    type ScanRunServiceFactoryDeps,
} from './scanRun.service';
import type { ScanRunRepository } from '@/src/persistence/scanRun';
import type { ScanDto } from '@/src/transport/scan.dto';
import { createScanRunFixture } from '@/tests/fixtures/scanRuns';
import type { ScanRunEntity } from '@/src/domain/scanRun.entity';
import type { ScanRunDto } from '@/src/transport/scanRun.dto';

describe('scanRun.service', () => {
    const expectedCursor = 'cursor';
    const expectedScanId = '019d29e9-eaed-7782-b537-fa2e7a73a3e4';
    const expectedUserId = '019c9abc-a10c-76e3-8287-885036664a5c';

    const expectedParent: Partial<ScanDto> = {
        id: expectedScanId,
    };
    const mockGetScanRun = mock();
    const mockListScanRuns = mock();
    const mockEncodeCursor = mock();
    const mockDecodeCursor = mock();
    const mockScanRunRepository: ScanRunRepository = {
        getScanRun: mockGetScanRun,
        listScanRuns: mockListScanRuns,
    };
    const expectedScanServiceFactoryDeps: ScanRunServiceFactoryDeps = {
        scanRunRepository: mockScanRunRepository,
        encodeCursor: mockEncodeCursor,
        decodeCursor: mockDecodeCursor,
    };
    const expectedScanRunId = 'id';

    afterEach(() => {
        mockEncodeCursor.mockReset();
        mockDecodeCursor.mockReset();
        mockGetScanRun.mockReset();
        mockListScanRuns.mockReset();
    });

    test('createScanRunService', () => {
        // Arrange
        const expectedScanRunService: ScanRunService = {
            getScanRun: expect.any(Function),
            listScanRuns: expect.any(Function),
        };
        // Act
        const actualScanRunService: ScanRunService = createScanRunService(
            expectedScanServiceFactoryDeps,
        );
        // Assert
        expect(actualScanRunService).toEqual(expectedScanRunService);
    });

    test('getScanRun', async () => {
        // Arrange
        const expectedGetScanRunArgs: GetScanRunArgs = {
            id: expectedScanRunId,
        };
        const expectedScanRunEntity: ScanRunEntity = createScanRunFixture(
            expectedScanId,
            expectedScanRunId,
        );
        const expectedScanRun: ScanRunDto = {
            id: expectedScanRunId,
            scan_id: expectedScanId,
            created_at: expectedScanRunEntity.created_at,
        };
        mockGetScanRun.mockResolvedValue(expectedScanRunEntity);
        const scanRunService: ScanRunService = createScanRunService(expectedScanServiceFactoryDeps);
        // Act
        const actualScanRun: ScanRunDto = await scanRunService.getScanRun(
            null,
            expectedGetScanRunArgs,
        );
        // Assert
        expect(actualScanRun).toEqual(expectedScanRun);
    });

    describe('listScanRuns', () => {
        test('valid scanId', async () => {
            // Arrange
            const expectedFirst = 15;
            const expectedScanRun: ScanRunDto = {
                id: expectedScanRunId,
                scan_id: expectedScanId,
                created_at: expect.any(Date),
            };
            const expectedArgs: ListScanRunsArgs = {
                first: expectedFirst,
                after: expectedCursor,
            };
            const expectedContext: GraphQlContext = {
                user: { id: expectedUserId },
            };
            const expectedScanEntity = createScanRunFixture(expectedScanId, expectedScanRunId);
            const expectedScanRunEntities: Array<ScanRunEntity> = [expectedScanEntity];
            const expectedHasNextPage = true;
            const expectedPageInfo: PageInfo = {
                hasNextPage: expectedHasNextPage,
                hasPreviousPage: false,
                startCursor: expectedCursor,
                endCursor: expectedCursor,
            };
            const expectedEdges: Array<Edge<ScanRunDto>> = [
                {
                    node: expectedScanRun,
                    cursor: expectedCursor,
                },
            ];
            mockListScanRuns.mockResolvedValue({
                scanRuns: expectedScanRunEntities,
                hasNextPage: expectedHasNextPage,
            });
            mockEncodeCursor.mockReturnValue(expectedCursor);
            const scanService: ScanRunService = createScanRunService(
                expectedScanServiceFactoryDeps,
            );
            // Act
            const actualScans: Pagination<Edge<ScanRunDto>> = await scanService.listScanRuns(
                expectedParent as unknown as ScanDto,
                expectedArgs,
                expectedContext,
            );
            // Assert
            expect(actualScans.edges).toEqual(expectedEdges);
            expect(actualScans.pageInfo).toEqual(expectedPageInfo);
            expect(mockListScanRuns).toHaveBeenCalledWith(expectedScanId);
        });
    });
});
