import type { BunRequest } from 'bun';
import { describe, test, expect, mock } from 'bun:test';
import { HEADERS } from '../../constants';
import { post } from './register';

describe('register', () => {
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
