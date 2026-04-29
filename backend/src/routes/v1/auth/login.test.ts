import { describe, test, expect, mock } from 'bun:test';
import container from '@/src/container.ts';
import { asFunction, asValue } from 'awilix';
import { API_LOGIN_ENDPOINT_V1 } from '@/src/constants';
import createLoginRoutes from './login';
import {
    type CsrfDecorators,
    type CsrfDecoratorsFactoryDeps,
} from '@/src/controllers/decorators/withCsrf';
import type { AnonymousCsrfStrategy } from '@/src/controllers/strategies/AnonymousCsrfStrategy';

describe('login', async () => {
    test('/api/v1/login', async () => {
        // Arrange
        const mockLoginController = {
            post: mock(),
        };
        const mockPreflightController = mock();
        const mockLoginControllerWithErrorHandling = mock();
        const mockControllerWithCors = mock();
        const mockControllerWithCsrf = mock();
        const mockAnonymousCsrfStrategy = {} as unknown as AnonymousCsrfStrategy;
        const mockCsrfDecoratorFactoryDeps: CsrfDecoratorsFactoryDeps<void> = {
            csrfStrategy: mockAnonymousCsrfStrategy,
        };
        const mockCreateLoginController = mock().mockReturnValue(mockLoginController);
        const mockWithErrorHandling = mock().mockReturnValue(mockLoginControllerWithErrorHandling);
        const mockWithCors = mock().mockReturnValue(mockControllerWithCors);
        const mockWithCsrf = mock().mockReturnValue(mockControllerWithCsrf);
        const mockCsrfDecorators: CsrfDecorators<void> = {
            withCsrf: mockWithCsrf,
        };
        const mockCreateCsrfDecorators = mock().mockReturnValue(mockCsrfDecorators);
        container.register({
            loginController: asFunction(mockCreateLoginController),
            preflightController: asValue(mockPreflightController),
            withErrorHandling: asValue(mockWithErrorHandling),
            withCors: asValue(mockWithCors),
            anonymousCsrfStrategy: asValue(mockAnonymousCsrfStrategy),
            createCsrfDecorators: asValue(mockCreateCsrfDecorators),
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
        expect(mockCreateCsrfDecorators).toHaveBeenCalledWith(mockCsrfDecoratorFactoryDeps);
        expect(mockWithCsrf).toHaveBeenCalledWith(mockLoginController.post);
        expect(mockWithErrorHandling).toHaveBeenCalledWith(mockControllerWithCsrf);
        expect(mockWithCors).toHaveBeenNthCalledWith(1, mockLoginControllerWithErrorHandling);
        expect(mockWithCors).toHaveBeenNthCalledWith(2, mockPreflightController);
        expect(actualRoute).toEqual(expectedRoute);
    });
});
