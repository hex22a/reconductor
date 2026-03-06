import { describe, expect, mock, test } from 'bun:test';
import { createMeRoutes } from './me';
import { API_ME_ENDPOINT_V1 } from '@/src/constants';
import container from '@/src/container';
import { asValue } from 'awilix';
import type { SessionStrategy } from '@/src/auth/strategies/SessionStrategy';
import type { HandleWithContextStrategy } from '@/src/auth/strategies/HandleWithContextStrategy';
import { type AuthDecorators, type AuthDecoratorsFactoryDeps } from '@/src/auth/decorators/auth';
import type { RequestContext } from '@/src/controllers/types';

describe('me', () => {
    test('/api/v1/me', () => {
        // Arrange
        const mockMeController = mock();
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
        const mockAuthDecorators: AuthDecorators = {
            withAuth: mock().mockReturnValue(mockHandlerWithAuth),
        };
        const mockCreateAuthDecorators = mock().mockReturnValue(mockAuthDecorators);
        const mockWithCors = mock().mockReturnValue(mockControllerWithCors);
        container.register({
            sessionStrategy: asValue(expectedSessionStrategy),
            heandleWithContextStrategy: asValue(expectedHandleWithContextStrategy),
            createAuthDecorators: asValue(mockCreateAuthDecorators),
            meController: asValue(mockMeController),
            preflightController: asValue(mockPreflightController),
            withCors: asValue(mockWithCors),
        });
        const expectedMeRoutes = {
            [API_ME_ENDPOINT_V1]: {
                GET: mockControllerWithCors,
                OPTIONS: mockControllerWithCors,
            },
        };
        // Act
        const actualMeRoutes = createMeRoutes();
        // Assert
        expect(mockCreateAuthDecorators).toHaveBeenLastCalledWith(expectedAuthDecoratorsDeps);
        expect(mockAuthDecorators.withAuth).toHaveBeenCalledWith(mockMeController);
        expect(mockWithCors).toHaveBeenNthCalledWith(1, mockHandlerWithAuth);
        expect(mockWithCors).toHaveBeenNthCalledWith(2, mockPreflightController);
        expect(actualMeRoutes).toEqual(expectedMeRoutes);
    });
});
