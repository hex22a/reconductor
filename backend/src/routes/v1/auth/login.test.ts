import { describe, test, expect, mock } from 'bun:test';
import container from '@/src/container.ts';
import { asFunction } from 'awilix';
import { API_LOGIN_ENDPOINT_V1 } from '@/src/constants';
import createLoginRoutes from './login';

describe('login', async () => {
    test('/api/v1/login', async () => {
        // Arrange
        const mockLoginController = {
            post: mock(),
        };
        const mockCreateLoginController = mock().mockReturnValue(mockLoginController);
        container.register({
            loginController: asFunction(mockCreateLoginController),
        })
        const expectedMethod = 'POST';
        const expectedRoute = {
            [API_LOGIN_ENDPOINT_V1]: { [expectedMethod]: mockLoginController.post }
        };
        const mockWithErrorHandling = mock((fn) => fn);
        // Act
        const actualRoute = createLoginRoutes(mockWithErrorHandling);
        // Assert
        expect(mockWithErrorHandling).toHaveBeenCalledWith(mockLoginController.post);
        expect(actualRoute).toEqual(expectedRoute);
    });
});
