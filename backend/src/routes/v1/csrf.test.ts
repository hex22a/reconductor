import container from '@/src/container';
import { asValue } from 'awilix';
import { describe, expect, mock, test } from 'bun:test';
import { API_CSRF_CONTROLLER_V1 } from '@/src/constants';
import { createCsrfRoutes } from './csrf';
import type { CsrfController } from '@/src/controllers/csrf';

describe('csrf', async () => {
    test('/api/v1/csrf', async () => {
        // Arrange
        const mockGetToken = mock();
        const mockCsrfController: CsrfController = {
            getToken: mockGetToken,
        };
        const mockPreflightController = mock();
        const mockCsrfControllerWithErrorHandling = mock();
        const mockControllerWithCors = mock();
        const mockWithErrorHandling = mock().mockReturnValue(mockCsrfControllerWithErrorHandling);
        const mockWithCors = mock().mockReturnValue(mockControllerWithCors);
        container.register({
            csrfController: asValue(mockCsrfController),
            preflightController: asValue(mockPreflightController),
            withCors: asValue(mockWithCors),
            withErrorHandling: asValue(mockWithErrorHandling),
        });
        const expectedRoutes = {
            [API_CSRF_CONTROLLER_V1]: {
                GET: mockControllerWithCors,
                OPTIONS: mockControllerWithCors,
            },
        };
        // Act
        const actualRoutes = createCsrfRoutes();
        // Assert
        expect(mockWithErrorHandling).toHaveBeenCalledWith(mockGetToken);
        expect(mockWithCors).toHaveBeenNthCalledWith(1, mockCsrfControllerWithErrorHandling);
        expect(mockWithCors).toHaveBeenNthCalledWith(2, mockPreflightController);
        expect(actualRoutes).toEqual(expectedRoutes);
    });
});
