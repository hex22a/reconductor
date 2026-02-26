import { afterEach, describe, expect, mock, test } from 'bun:test';
import { createGenerateRandomToken, type GenerateRandomTokenFactoryDeps } from './random';
import { TOKEN_RANDOM_BYTES_ARRAY_LENGTH } from '../constants';

describe('random', () => {
    const mockGetRandomValues = mock();
    const mockCrypto = {
        getRandomValues: mockGetRandomValues,
    } satisfies Partial<Crypto>;
    const expectedGenerateRandomTokenDeps: GenerateRandomTokenFactoryDeps = {
        cryptoProvider: mockCrypto as unknown as Crypto,
    };

    afterEach(() => {
        mockGetRandomValues.mockReset();
    });

    test('createGenerateRandomToken', () => {
        // Arrange
        // Act
        const generateRandomToken = createGenerateRandomToken(expectedGenerateRandomTokenDeps);
        // Assert
        expect(generateRandomToken).toBeFunction();
    });

    test('generateRadomToken', () => {
        // Arrange
        const expectedRandomBytes: Uint8Array = Uint8Array.from([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        const expectedRandomToken = Buffer.from(expectedRandomBytes).toString('base64url');
        const expectedBuffer = new Uint8Array(TOKEN_RANDOM_BYTES_ARRAY_LENGTH);
        const generateRandomToken = createGenerateRandomToken(expectedGenerateRandomTokenDeps);
        mockGetRandomValues.mockReturnValue(expectedRandomBytes);
        // Act
        const actualRandomToken = generateRandomToken();
        // Assert
        expect(actualRandomToken).toEqual(expectedRandomToken);
        expect(mockGetRandomValues).toHaveBeenCalledWith(expectedBuffer);
    });
});
