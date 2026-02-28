import { describe, expect, test } from 'bun:test';
import { constants } from 'node:http2';
import { preflight } from './preflight';

describe('preflight', () => {
    test('returns 204', () => {
        // Arrange
        const expectedResponse = new Response(null, { status: constants.HTTP_STATUS_NO_CONTENT });
        // Act
        const actualResponse: Response = preflight();
        // Assert
        expect(actualResponse.status).toEqual(expectedResponse.status);
    });
});
