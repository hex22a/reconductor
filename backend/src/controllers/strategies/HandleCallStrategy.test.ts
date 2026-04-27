import type { BunRequest } from 'bun';
import { describe, expect, it, mock } from 'bun:test';
import { HandleCallStrategy } from './HandleCallStrategy';

describe('HandleCallStrategy', () => {
    it('calls handler', async () => {
        // Arrange
        const mockHandler = mock();
        const expectedResponseJson = { respose: 'some valid response' };
        const expectedResponse: Response = Response.json(expectedResponseJson);
        const expectedRequest = {} satisfies Partial<BunRequest>;
        mockHandler.mockResolvedValue(expectedResponse);
        const handleWithContextStrategy = new HandleCallStrategy();
        // Act
        const actualResponse: Response = await handleWithContextStrategy.handle(
            mockHandler,
            expectedRequest as unknown as BunRequest,
        );
        // Assert
        expect(mockHandler).toHaveBeenCalledWith(expectedRequest);
        expect(await actualResponse.json()).toEqual(expectedResponseJson);
        expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
        expect(actualResponse.status).toEqual(expectedResponse.status);
    });
});
