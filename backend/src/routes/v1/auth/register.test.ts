import { describe, test, expect, mock } from 'bun:test';
import createRegisterRoutes from './register.ts';
import { API_REGISTER_ENDPOINT_V1 } from '../../../constants.ts';
import container from '@/src/container.ts';
import { asFunction, asValue } from 'awilix';

describe('register', async () => {
    test('/api/v1/register', async () => {
        // Arrange
        const mockRegisterController = {
            post: mock(),
        };
        const mockPreflightController = mock();
        const mockRegisterControllerWithErrorHandling = mock();
        const mockControllerWithCors = mock();
        const mockCreateRegisterController = mock().mockReturnValue(mockRegisterController);
        const mockWithErrorHandling = mock().mockReturnValue(
            mockRegisterControllerWithErrorHandling,
        );
        const mockWithCors = mock().mockReturnValue(mockControllerWithCors);
        container.register({
            registerController: asFunction(mockCreateRegisterController),
            preflightController: asValue(mockPreflightController),
            withErrorHandling: asValue(mockWithErrorHandling),
            withCors: asValue(mockWithCors),
        });
        const expectedUrl = API_REGISTER_ENDPOINT_V1;
        const expectedRoute = {
            [expectedUrl]: {
                POST: mockControllerWithCors,
                OPTIONS: mockControllerWithCors,
            },
        };
        // Act
        const actualRoute = createRegisterRoutes();
        // Assert
        expect(mockWithErrorHandling).toHaveBeenCalledWith(mockRegisterController.post);
        expect(mockWithCors).toHaveBeenNthCalledWith(1, mockRegisterControllerWithErrorHandling);
        expect(mockWithCors).toHaveBeenNthCalledWith(2, mockPreflightController);
        expect(actualRoute).toEqual(expectedRoute);
    });
});
