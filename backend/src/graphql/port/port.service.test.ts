import { afterEach, describe, expect, mock, test } from 'bun:test';
import type { GraphQlContext } from '@/src/transport/graphql.context';
import type { PageInfo } from '@/src/transport/pageInfo.dto';
import type { Edge } from '@/src/transport/edge.dto';
import type { Pagination } from '@/src/transport/pagination.dto';
import {
    createPortService,
    type PortService,
    type PortServiceFactoryDeps,
    type ListPortsArgs,
} from './port.service';
import type { HostDto } from '@/src/transport/host.dto';
import type { PortRepository } from '@/src/persistence/port.db';
import { createPortFixture } from '@/tests/fixtures/ports';
import type { PortDto } from '@/src/transport/port.dto';
import type { PortEntity } from '@/src/domain/port.entity';

describe('port.service', () => {
    const expectedCursor = 'cursor';
    const expectedHostId = '019d29e9-eaed-7782-b537-fa2e7a73a3e4';
    const expectedUserId = '019c9abc-a10c-76e3-8287-885036664a5c';

    const expectedParent: Partial<HostDto> = {
        id: expectedHostId,
    };
    const mockGetPort = mock();
    const mockListPorts = mock();
    const mockEncodeCursor = mock();
    const mockDecodeCursor = mock();
    const mockHostRepository: PortRepository = {
        getPort: mockGetPort,
        listPorts: mockListPorts,
    };
    const expectedHostServiceFactoryDeps: PortServiceFactoryDeps = {
        portRepository: mockHostRepository,
        encodeCursor: mockEncodeCursor,
        decodeCursor: mockDecodeCursor,
    };
    const expectedPortId = 'id';

    afterEach(() => {
        mockEncodeCursor.mockReset();
        mockDecodeCursor.mockReset();
        mockGetPort.mockReset();
        mockListPorts.mockReset();
    });

    test('createPortService', () => {
        // Arrange
        const expectedHostService: PortService = {
            listPorts: expect.any(Function),
        };
        // Act
        const actualHostService: PortService = createPortService(expectedHostServiceFactoryDeps);
        // Assert
        expect(actualHostService).toEqual(expectedHostService);
    });

    describe('listHosts', () => {
        test('valid scanRunId', async () => {
            // Arrange
            const expectedPortNumber = 22;
            const expectedFirst = 15;
            const expectedPort: PortDto = {
                id: expectedPortId,
                port: expectedPortNumber,
            };
            const expectedArgs: ListPortsArgs = {
                first: expectedFirst,
                after: expectedCursor,
            };
            const expectedContext: GraphQlContext = {
                user: { id: expectedUserId },
            };
            const [expectedPortEntity] = createPortFixture(
                expectedHostId,
                expectedPortNumber,
                expectedPortId,
            );
            const expectedPortEntities: Array<PortEntity> = [expectedPortEntity];
            const expectedHasNextPage = true;
            const expectedPageInfo: PageInfo = {
                hasNextPage: expectedHasNextPage,
                hasPreviousPage: false,
                startCursor: expectedCursor,
                endCursor: expectedCursor,
            };
            const expectedEdges: Array<Edge<HostDto>> = [
                {
                    node: expectedPort,
                    cursor: expectedCursor,
                },
            ];
            mockListPorts.mockResolvedValue({
                ports: expectedPortEntities,
                hasNextPage: expectedHasNextPage,
            });
            mockEncodeCursor.mockReturnValue(expectedCursor);
            const portService: PortService = createPortService(expectedHostServiceFactoryDeps);
            // Act
            const actualPorts: Pagination<Edge<HostDto>> = await portService.listPorts(
                expectedParent as unknown as HostDto,
                expectedArgs,
                expectedContext,
            );
            // Assert
            expect(actualPorts.edges).toEqual(expectedEdges);
            expect(actualPorts.pageInfo).toEqual(expectedPageInfo);
            expect(mockListPorts).toHaveBeenCalledWith(expectedHostId);
        });
    });
});
