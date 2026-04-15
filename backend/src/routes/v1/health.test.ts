import { API_HEALTH_ENDPOINT_V1 } from '@/src/constants';
import container from '@/src/container';
import { asValue } from 'awilix';
import { describe, expect, mock, test } from 'bun:test';
import createHealthRoutes from './health';

describe('health', async () => {
    test('/api/v1/health', async () => {
        // Arrange
        const mockHealthController = mock();
        const mockPreflightController = mock();
        const mockControllerWithCors = mock();
        const mockWithCors = mock().mockReturnValue(mockControllerWithCors);
        container.register({
            healthController: asValue(mockHealthController),
            preflightController: asValue(mockPreflightController),
            withCors: asValue(mockWithCors),
        });
        const expectedRoute = {
            [API_HEALTH_ENDPOINT_V1]: {
                GET: mockControllerWithCors,
                OPTIONS: mockControllerWithCors,
            },
        };
        // Act
        const actualRoute = createHealthRoutes();
        // Assert
        expect(mockWithCors).toHaveBeenNthCalledWith(1, mockHealthController);
        expect(mockWithCors).toHaveBeenNthCalledWith(2, mockPreflightController);
        expect(actualRoute).toEqual(expectedRoute);
    });
});
