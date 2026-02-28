import { describe, test, expect, mock } from 'bun:test';
import container from '@/src/container.ts';
import { asFunction, asValue } from 'awilix';
import { API_LOGIN_ENDPOINT_V1 } from '@/src/constants';
import createLoginRoutes from './login';

describe('login', async () => {
    test('/api/v1/login', async () => {
        // Arrange
        const mockLoginController = {
            post: mock(),
        };
        const mockPreflightController = mock();
        const mockLoginControllerWithErrorHandling = mock();
        const mockControllerWithCors = mock();
        const mockCreateLoginController = mock().mockReturnValue(mockLoginController);
        const mockWithErrorHandling = mock().mockReturnValue(mockLoginControllerWithErrorHandling);
        const mockWithCors = mock().mockReturnValue(mockControllerWithCors);
        container.register({
            loginController: asFunction(mockCreateLoginController),
            preflightController: asValue(mockPreflightController),
            withErrorHandling: asValue(mockWithErrorHandling),
            withCors: asValue(mockWithCors),
        });
        const expectedRoute = {
            [API_LOGIN_ENDPOINT_V1]: {
                POST: mockControllerWithCors,
                OPTIONS: mockControllerWithCors,
            },
        };
        // Act
        const actualRoute = createLoginRoutes();
        // Assert
        expect(mockWithErrorHandling).toHaveBeenCalledWith(mockLoginController.post);
        expect(mockWithCors).toHaveBeenNthCalledWith(1, mockLoginControllerWithErrorHandling);
        expect(mockWithCors).toHaveBeenNthCalledWith(2, mockPreflightController);
        expect(actualRoute).toEqual(expectedRoute);
    });
});
