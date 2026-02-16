import { describe, test, expect, mock } from 'bun:test';
import createRegisterRoutes from './register.ts';
import { API_REGISTER_ENDPOINT_V1 } from '../../../constants.ts';
import container from '@/src/container.ts';
import { asFunction } from 'awilix';

describe('register', async () => {
    test('/api/v1/register', async () => {
        // Arrange
        const mockRegisterController = {
            post: mock(),
        };
        const mockCreateRegisterController = mock().mockReturnValue(mockRegisterController);
        container.register({
            registerController: asFunction(mockCreateRegisterController),
        })
        const expectedMethod = 'POST';
        const expectedUrl = API_REGISTER_ENDPOINT_V1;
        const expectedRoute = {
            [expectedUrl]: { [expectedMethod]: mockRegisterController.post }
        };
        // Act
        const actualRoute = createRegisterRoutes();
        // Assert
        expect(actualRoute).toEqual(expectedRoute);
    });
});
