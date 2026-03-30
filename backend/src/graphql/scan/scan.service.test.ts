import { afterEach, describe, expect, mock, test } from 'bun:test';
import type { ScanRepository } from '@/src/persistence/scan.db';
import {
    createScanService,
    type CreateScanArgs,
    type ListScansArgs,
    type ScanService,
    type ScanServiceFactoryDeps,
} from './scan.service';
import type { ScanDto } from '@/src/transport/scan.dto';
import type { GraphQlContext } from '@/src/transport/graphql.context';
import { createScanFixture } from '@/tests/fixtures/scans';
import type { ScanEntity } from '@/src/domain/scan.entity';
import type { PageInfo } from '@/src/transport/pageInfo.dto';
import type { Edge } from '@/src/transport/edge.dto';
import type { Pagination } from '@/src/transport/pagination.dto';
import type { ValidationError } from '@/src/transport/error.dto';
import type { CreateEntityPayload } from '@/src/transport/payload.dto';

describe('scan.service', () => {
    const expectedCursor = 'cursor';
    const expectedProjectId = '019d29e9-eaed-7782-b537-fa2e7a73a3e4';
    const expectedUserId = '019c9abc-a10c-76e3-8287-885036664a5c';

    const expectedParent = null;
    const mockCreateScan = mock();
    const mockGetScan = mock();
    const mockListScans = mock();
    const mockEncodeCursor = mock();
    const mockDecodeCursor = mock();
    const mockScanRepository: ScanRepository = {
        getScan: mockGetScan,
        createScan: mockCreateScan,
        listScans: mockListScans,
    };
    const expectedScanServiceFactoryDeps: ScanServiceFactoryDeps = {
        scanRepository: mockScanRepository,
        encodeCursor: mockEncodeCursor,
        decodeCursor: mockDecodeCursor,
    };
    const expectedScanId = 'id';

    afterEach(() => {
        mockEncodeCursor.mockReset();
        mockDecodeCursor.mockReset();
        mockCreateScan.mockReset();
        mockGetScan.mockReset();
        mockListScans.mockReset();
    });

    test('createScanService', () => {
        // Arrange
        const expectedScanService: ScanService = {
            listScans: expect.any(Function),
            createScan: expect.any(Function),
        };
        // Act
        const actualScanService: ScanService = createScanService(expectedScanServiceFactoryDeps);
        // Assert
        expect(actualScanService).toEqual(expectedScanService);
    });

    describe('listScans', () => {
        test('valid projectId', async () => {
            // Arrange
            const expectedTarget = '192.168.50.0/16';
            const expectedScan: ScanDto = {
                id: expectedScanId,
                created_at: expect.any(Date),
                target: expectedTarget,
            };
            const expectedArgs: ListScansArgs = {
                projectId: expectedProjectId,
            };
            const expectedContext: GraphQlContext = {
                user: { id: expectedUserId },
            };
            const [expectedScanEntity] = createScanFixture(
                expectedProjectId,
                expectedTarget,
                undefined,
                expectedScanId,
            );
            const expectedScanEntities: Array<ScanEntity> = [expectedScanEntity];
            const expectedHasNextPage = true;
            const expectedPageInfo: PageInfo = {
                hasNextPage: expectedHasNextPage,
                hasPreviousPage: false,
                startCursor: expectedCursor,
                endCursor: expectedCursor,
            };
            const expectedEdges: Array<Edge<ScanDto>> = [
                {
                    node: expectedScan,
                    cursor: expectedCursor,
                },
            ];
            mockListScans.mockResolvedValue({
                scans: expectedScanEntities,
                hasNextPage: expectedHasNextPage,
            });
            mockEncodeCursor.mockReturnValue(expectedCursor);
            const scanService: ScanService = createScanService(expectedScanServiceFactoryDeps);
            // Act
            const actualScans: Pagination<Edge<ScanDto>> = await scanService.listScans(
                expectedParent,
                expectedArgs,
                expectedContext,
            );
            // Assert
            expect(actualScans.edges).toEqual(expectedEdges);
            expect(actualScans.pageInfo).toEqual(expectedPageInfo);
            expect(mockListScans).toHaveBeenCalledWith(expectedProjectId);
        });
    });

    describe('createScan', () => {
        test('ad-hoc (no schedule)', async () => {
            // Arrange
            const expectedTarget = '192.168.50.0/16';
            const expectedValidationErrors: Array<ValidationError> = [];
            const expectedScan: ScanDto = {
                id: expectedScanId,
                created_at: expect.any(Date),
                target: expectedTarget,
            };
            const expectedArgs: CreateScanArgs = {
                input: {
                    target: expectedTarget,
                    projectId: expectedProjectId,
                },
            };
            const expectedContext: GraphQlContext = {
                user: { id: expectedUserId },
            };
            const [expectedScanEntity, expectedScanInsert] = createScanFixture(
                expectedProjectId,
                expectedTarget,
                undefined,
                expectedScanId,
            );
            mockCreateScan.mockResolvedValue(expectedScanEntity);
            mockEncodeCursor.mockReturnValue(expectedCursor);
            const scanService: ScanService = createScanService(expectedScanServiceFactoryDeps);
            // Act
            const actualCreateScanPayload: CreateEntityPayload<Edge<ScanDto>> =
                await scanService.createScan(expectedParent, expectedArgs, expectedContext);
            // Assert
            expect(actualCreateScanPayload.edge!.node).toEqual(expectedScan);
            expect(actualCreateScanPayload.edge!.cursor).toEqual(expectedCursor);
            expect(actualCreateScanPayload.errors).toEqual(expectedValidationErrors);
            expect(mockCreateScan).toHaveBeenCalledWith(expectedScanInsert);
        });
    });
});
