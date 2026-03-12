import { describe, expect, mock, test } from 'bun:test';
import { createLogoutRoutes } from './logout';
import type { SessionStrategy } from '@/src/auth/strategies/SessionStrategy';
import type { HandleWithContextStrategy } from '@/src/auth/strategies/HandleWithContextStrategy';
import type { AuthDecorators, AuthDecoratorsFactoryDeps } from '@/src/auth/decorators/auth';
import type { RequestContext } from '@/src/controllers/types';
import container from '@/src/container';
import { asValue } from 'awilix';
import { API_LOGOUT_ENDPOINT_V1 } from '@/src/constants';

describe('/api/v1/logout', () => {
    test('createLogoutRoutes', () => {
        // Arrange
        const mockLogoutController = mock();
        const mockHandlerWithAuth = mock();
        const mockPreflightController = mock();
        const mockControllerWithCors = mock();
        const expectedSessionStrategy: SessionStrategy = {} as unknown as SessionStrategy;
        const expectedHandleWithContextStrategy: HandleWithContextStrategy =
            {} as unknown as HandleWithContextStrategy;
        const expectedAuthDecoratorsDeps: AuthDecoratorsFactoryDeps<RequestContext> = {
            authStrategy: expectedSessionStrategy,
            handleStrategy: expectedHandleWithContextStrategy,
        };
        const mockAuthDecorators: AuthDecorators<RequestContext> = {
            withAuth: mock().mockReturnValue(mockHandlerWithAuth),
        };
        const mockCreateAuthDecorators = mock().mockReturnValue(mockAuthDecorators);
        const mockWithCors = mock().mockReturnValue(mockControllerWithCors);
        container.register({
            sessionStrategy: asValue(expectedSessionStrategy),
            heandleWithContextStrategy: asValue(expectedHandleWithContextStrategy),
            createAuthDecorators: asValue(mockCreateAuthDecorators),
            logoutController: asValue(mockLogoutController),
            preflightController: asValue(mockPreflightController),
            withCors: asValue(mockWithCors),
        });
        const expectedLogoutRoutes = {
            [API_LOGOUT_ENDPOINT_V1]: {
                POST: mockControllerWithCors,
                OPTIONS: mockControllerWithCors,
            },
        };
        // Act
        const actualLogoutRoutes = createLogoutRoutes();
        // Assert
        expect(mockCreateAuthDecorators).toHaveBeenLastCalledWith(expectedAuthDecoratorsDeps);
        expect(mockAuthDecorators.withAuth).toHaveBeenCalledWith(mockLogoutController);
        expect(mockWithCors).toHaveBeenNthCalledWith(1, mockHandlerWithAuth);
        expect(mockWithCors).toHaveBeenNthCalledWith(2, mockPreflightController);
        expect(actualLogoutRoutes).toEqual(expectedLogoutRoutes);
    });
});
