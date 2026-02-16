import type { BunRequest } from 'bun';
import { constants } from 'node:http2';
import { describe, test, expect, mock } from 'bun:test';
import { HEADERS, UNEXPECTED_END_OF_JSON_ERROR_MESSAGE, UNEXPECTED_ERROR_MESSAGE } from '../../constants';
import { post } from './register';

describe('register', () => {
    test('returns 500 for unexpected error', async () => {
        // Arrange
        const expectedResponseJson = {
            error: UNEXPECTED_ERROR_MESSAGE,
        };
        const expectedResponseInit: ResponseInit = {
            headers: HEADERS,
            status: constants.HTTP_STATUS_INTERNAL_SERVER_ERROR,
        };
        const expectedResponse: Response = Response.json(expectedResponseJson, expectedResponseInit);

        const expectedRequest = {
            json: mock().mockRejectedValue(Symbol('UNEXPECTED')),
        } satisfies Partial<BunRequest>;

        // Act
        const actualResponse: Response = await post(expectedRequest as unknown as BunRequest);

        // Assert
        expect(expectedRequest.json).toHaveBeenCalled();
        expect(await actualResponse.json()).toEqual(expectedResponseJson);
        expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
        expect(actualResponse.status).toEqual(expectedResponse.status);
    });

    test('returns 400 when no JSON provided', async () => {
        // Arrange
        const expectedResponseJson = {
            error: UNEXPECTED_END_OF_JSON_ERROR_MESSAGE,
        };
        const expectedResponseInit: ResponseInit = {
            headers: HEADERS,
            status: constants.HTTP_STATUS_BAD_REQUEST
        };
        const expectedResponse: Response = Response.json(expectedResponseJson, expectedResponseInit);

        const expectedRequest = {
            json: mock().mockRejectedValue(new Error(UNEXPECTED_END_OF_JSON_ERROR_MESSAGE)),
        } satisfies Partial<BunRequest>;

        // Act
        const actualResponse: Response = await post(expectedRequest as unknown as BunRequest);

        // Assert
        expect(expectedRequest.json).toHaveBeenCalled();
        expect(await actualResponse.json()).toEqual(expectedResponseJson);
        expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
        expect(actualResponse.status).toEqual(expectedResponse.status);
    });

    test('POST with valid username and password', async () => {
        // Arrange
        const expectedUsername = 'username';
        const expectedPassword = 'password';
        const expectedResponse: Response = Response.json({ foo: 'bar' }, { headers: HEADERS });
        const expectedRequestJson = { username: expectedUsername, password: expectedPassword };

        const expectedRequest = { json: mock().mockResolvedValue(expectedRequestJson) } satisfies Partial<BunRequest>;

        // Act
        const actualResponse: Response = await post(expectedRequest as unknown as BunRequest);

        // Assert
        expect(expectedRequest.json).toHaveBeenCalled();
        expect(actualResponse).toEqual(expectedResponse);
    });
});
