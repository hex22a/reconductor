import { afterEach, describe, expect, mock, test, it } from 'bun:test';
import type { RequestHandler } from '@/src/controllers/types';
import type { BunRequest } from 'bun';
import { toHandler } from './FetchToHandlerAdapter';

describe('FetchToHandlerAdapter', () => {
    const mockFetch = mock();

    afterEach(() => {
        mockFetch.mockReset();
    });

    test('toHandler', () => {
        // Arrange
        // Act
        const actualHandler: RequestHandler<void> = toHandler(mockFetch);
        // Assert
        expect(actualHandler).toBeFunction();
    });

    it('calls fetch with request', () => {
        // Arrange
        const expectedRequest = {} satisfies Partial<BunRequest>;
        const expectedResponseJson = {};
        const expectedResponse: Response = Response.json(expectedResponseJson);
        const handler: RequestHandler<void> = toHandler(mockFetch);
        mockFetch.mockReturnValue(expectedResponse);
        // Act
        const actualResponse = handler(expectedRequest as unknown as BunRequest);
        // Assert
        expect(actualResponse).toEqual(expectedResponse);
        expect(mockFetch).toHaveBeenLastCalledWith(expectedRequest);
    });
});
