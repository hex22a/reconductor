import { describe, expect, test } from 'bun:test';
import { constants } from 'node:http2';
import { health } from './health';

describe('health', () => {
    test('returns 200', async () => {
        // Arrange
        const expectedResponseJson = {
            healthy: true,
        };
        const expectedResponse = Response.json(expectedResponseJson, {
            status: constants.HTTP_STATUS_OK,
        });
        // Act
        const actualResponse: Response = health();
        // Assert
        expect(await actualResponse.json()).toEqual(expectedResponseJson);
        expect(actualResponse.status).toEqual(expectedResponse.status);
    });
});
