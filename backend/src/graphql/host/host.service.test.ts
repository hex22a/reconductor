import { afterEach, describe, expect, mock, test } from 'bun:test';
import type { GraphQlContext } from '@/src/transport/graphql.context';
import type { PageInfo } from '@/src/transport/pageInfo.dto';
import type { Edge } from '@/src/transport/edge.dto';
import type { Pagination } from '@/src/transport/pagination.dto';
import {
    createHostService,
    type HostService,
    type HostServiceFactoryDeps,
    type ListHostsArgs,
} from './host.service';
import type { ScanRunDto } from '@/src/transport/scanRun.dto';
import type { HostRepository } from '@/src/persistence/host.db';
import { createHostFixture } from '@/tests/fixtures/hosts';
import type { HostEntity } from '@/src/domain/host.entity';
import type { HostDto } from '@/src/transport/host.dto';

describe('host.service', () => {
    const expectedCursor = 'cursor';
    const expectedScanRunId = '019d29e9-eaed-7782-b537-fa2e7a73a3e4';
    const expectedUserId = '019c9abc-a10c-76e3-8287-885036664a5c';

    const expectedParent: Partial<ScanRunDto> = {
        id: expectedScanRunId,
    };
    const mockGetHost = mock();
    const mockListHosts = mock();
    const mockEncodeCursor = mock();
    const mockDecodeCursor = mock();
    const mockHostRepository: HostRepository = {
        getHost: mockGetHost,
        listHosts: mockListHosts,
    };
    const expectedHostServiceFactoryDeps: HostServiceFactoryDeps = {
        hostRepository: mockHostRepository,
        encodeCursor: mockEncodeCursor,
        decodeCursor: mockDecodeCursor,
    };
    const expectedHostId = 'id';

    afterEach(() => {
        mockEncodeCursor.mockReset();
        mockDecodeCursor.mockReset();
        mockGetHost.mockReset();
        mockListHosts.mockReset();
    });

    test('createHostService', () => {
        // Arrange
        const expectedHostService: HostService = {
            listHosts: expect.any(Function),
        };
        // Act
        const actualHostService: HostService = createHostService(expectedHostServiceFactoryDeps);
        // Assert
        expect(actualHostService).toEqual(expectedHostService);
    });

    describe('listHosts', () => {
        test('valid scanRunId', async () => {
            // Arrange
            const expectedIp = '192.168.0.1';
            const expectedFirst = 15;
            const expectedHost: HostDto = {
                id: expectedHostId,
                ip: expectedIp,
            };
            const expectedArgs: ListHostsArgs = {
                first: expectedFirst,
                after: expectedCursor,
            };
            const expectedContext: GraphQlContext = {
                user: { id: expectedUserId },
            };
            const [expectedHostEntity] = createHostFixture(
                expectedScanRunId,
                expectedIp,
                expectedHostId,
            );
            const expectedHostEntities: Array<HostEntity> = [expectedHostEntity];
            const expectedHasNextPage = true;
            const expectedPageInfo: PageInfo = {
                hasNextPage: expectedHasNextPage,
                hasPreviousPage: false,
                startCursor: expectedCursor,
                endCursor: expectedCursor,
            };
            const expectedEdges: Array<Edge<HostDto>> = [
                {
                    node: expectedHost,
                    cursor: expectedCursor,
                },
            ];
            mockListHosts.mockResolvedValue({
                hosts: expectedHostEntities,
                hasNextPage: expectedHasNextPage,
            });
            mockEncodeCursor.mockReturnValue(expectedCursor);
            const scanService: HostService = createHostService(expectedHostServiceFactoryDeps);
            // Act
            const actualScans: Pagination<Edge<HostDto>> = await scanService.listHosts(
                expectedParent as unknown as ScanRunDto,
                expectedArgs,
                expectedContext,
            );
            // Assert
            expect(actualScans.edges).toEqual(expectedEdges);
            expect(actualScans.pageInfo).toEqual(expectedPageInfo);
            expect(mockListHosts).toHaveBeenCalledWith(expectedScanRunId);
        });
    });
});
