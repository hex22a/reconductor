import type { BunRequest } from 'bun';
import { afterEach, describe, expect, mock, test } from 'bun:test';
import { withCors } from './withCors';
import type { RequestHandler } from '@/src/controllers/types';
import { CORS_HEADERS } from '@/src/constants';

describe('withCors', () => {
    const mockController = mock();

    afterEach(() => {
        mockController.mockReset();
    });

    test('adds cors headers', async () => {
        // Arrange
        const expectedResponseJson = { respose: 'some valid response' };
        const expectedCustomHeaders = {
            'x-recon-custom-header': 'custom',
        };
        const expectedHeadersJson = {
            'content-type': 'application/json;charset=utf-8',
            ...expectedCustomHeaders,
            ...CORS_HEADERS,
        };
        const expectedResponseInit: ResponseInit = {
            headers: expectedCustomHeaders,
        };
        const expectedResponse: Response = Response.json(
            expectedResponseJson,
            expectedResponseInit,
        );
        const expectedRequest = {} satisfies Partial<BunRequest>;
        mockController.mockResolvedValue(expectedResponse);
        const decoratedController: RequestHandler<void> = withCors(mockController);
        // Act
        const actualResponse: Response = await decoratedController(
            expectedRequest as unknown as BunRequest,
        );
        // Assert
        expect(mockController).toHaveBeenCalledWith(expectedRequest);
        expect(await actualResponse.json()).toEqual(expectedResponseJson);
        expect(actualResponse.headers.toJSON()).toEqual(expectedHeadersJson);
        expect(actualResponse.status).toEqual(expectedResponse.status);
    });
});
