import { afterEach, describe, expect, mock, test } from 'bun:test';
import {
    createPortResolver,
    type PortResolver,
    type PortResolverFactoryDeps,
} from './port.resolver';
import type { PortService } from './port.service';

describe('host.resolver', () => {
    const mockListPorts = mock();
    const mockPortService: PortService = {
        listPorts: mockListPorts,
    };
    const expectedPortResolverFactoryDeps: PortResolverFactoryDeps = {
        portService: mockPortService,
    };

    afterEach(() => {
        mockListPorts.mockReset();
    });

    test('createScanRunResolver', () => {
        // Arrange
        const expectedPortResolver: PortResolver = {
            Host: {
                ports: mockListPorts,
            },
        };
        // Act
        const actualPortResolver: PortResolver = createPortResolver(
            expectedPortResolverFactoryDeps,
        );
        // Assert
        expect(actualPortResolver).toEqual(expectedPortResolver);
    });
});
