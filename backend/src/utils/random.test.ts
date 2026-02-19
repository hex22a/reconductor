import { afterEach, describe, expect, mock, test } from 'bun:test';
import { createGenerateRandomToken } from './random';

describe('random', () => {
    const mockRandomBytes = mock();

    afterEach(() => {
        mockRandomBytes.mockReset();
    })

    test('createGenerateRandomToken', () => {
        // Arrange
        // Act
        const generateRandomToken = createGenerateRandomToken(mockRandomBytes);
        // Assert
        expect(generateRandomToken).toBeFunction();
    });

    test('generateRadomToken', () => {
        // Arrange
        const expectedRandomBytes: Uint8Array = Uint8Array.from([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        const expectedRandomToken = Buffer.from(expectedRandomBytes).toString('base64url');
        const generateRandomToken = createGenerateRandomToken(mockRandomBytes);
        mockRandomBytes.mockReturnValue(expectedRandomBytes);
        // Act
        const actualRandomToken = generateRandomToken();
        // Assert
        expect(actualRandomToken).toEqual(expectedRandomToken);
    })
});
