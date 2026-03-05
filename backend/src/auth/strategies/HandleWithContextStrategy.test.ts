import { describe, expect, it, mock } from 'bun:test';
import { HandleWithContextStrategy } from './HandleWithContextStrategy';
import type { BunRequest } from 'bun';
import type { RequestContext } from '@/src/controllers/types';
import { createUserSessionFixture } from '@/tests/fixtures/sessions';

describe('HandleWithContextStrategy', () => {
    it('calls handler with context', async () => {
        // Arrange
        const mockHandler = mock();
        const expectedToken = 'token';
        const expectedUserId = '019cb419-2d24-727d-b66a-cf6390891464';
        const expectedUsername = 'username';
        const [expectedUserSession] = createUserSessionFixture(
            expectedToken,
            expectedUserId,
            expectedUsername,
        );
        const expectedResponseJson = { respose: 'some valid response' };
        const expectedResponse: Response = Response.json(expectedResponseJson);
        const expectedRequest = {} satisfies Partial<BunRequest>;
        const expectedContext: RequestContext = {
            user: expectedUserSession,
        };
        mockHandler.mockResolvedValue(expectedResponse);
        const handleWithContextStrategy = new HandleWithContextStrategy();
        // Act
        const actualResponse: Response = await handleWithContextStrategy.handle(
            mockHandler,
            expectedRequest as unknown as BunRequest,
            expectedContext,
        );
        // Assert
        expect(mockHandler).toHaveBeenCalledWith(expectedRequest, expectedContext);
        expect(await actualResponse.json()).toEqual(expectedResponseJson);
        expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
        expect(actualResponse.status).toEqual(expectedResponse.status);
    });
});
