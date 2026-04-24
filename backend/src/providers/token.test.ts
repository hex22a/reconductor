import { afterEach, describe, expect, mock, test } from 'bun:test';
import { createTokenProvider, type TokenPorviderFactoryDeps, type TokenProvider } from './token';
import {
    CSRF_SECRET,
    TOKEN_RANDOM_BYTES_ARRAY_LENGTH,
    USER_SESSION_TTL_MILLISECONDS,
} from '../constants';
import type { CsrfProvider } from './csrf';

describe('random', () => {
    const mockGetRandomValues = mock();
    const mockGenerate = mock();
    const mockVerify = mock();
    const mockCrypto = {
        getRandomValues: mockGetRandomValues,
    } satisfies Partial<Crypto>;
    const mockCsrfProvider = {
        generate: mockGenerate,
        verify: mockVerify,
    } satisfies CsrfProvider;
    const expectedTokenPorviderDeps: TokenPorviderFactoryDeps = {
        cryptoProvider: mockCrypto as unknown as Crypto,
        csrfProvider: mockCsrfProvider,
    };

    afterEach(() => {
        mockGetRandomValues.mockReset();
        mockGenerate.mockReset();
        mockVerify.mockReset();
    });

    test('createTokenProvider', () => {
        // Arrange
        const actualTokenProvider: TokenProvider = {
            generateRandomToken: expect.any(Function),
            generateCsrfToken: expect.any(Function),
            verifyCsrfToken: expect.any(Function),
        };
        // Act
        const expectedTokenProvider: TokenProvider = createTokenProvider(expectedTokenPorviderDeps);
        // Assert
        expect(expectedTokenProvider).toEqual(actualTokenProvider);
    });

    test('generateRadomToken', () => {
        // Arrange
        const expectedRandomBytes: Uint8Array = Uint8Array.from([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        const expectedRandomToken = Buffer.from(expectedRandomBytes).toString('base64url');
        const expectedBuffer = new Uint8Array(TOKEN_RANDOM_BYTES_ARRAY_LENGTH);
        const tokenProvider: TokenProvider = createTokenProvider(expectedTokenPorviderDeps);
        mockGetRandomValues.mockReturnValue(expectedRandomBytes);
        // Act
        const actualRandomToken = tokenProvider.generateRandomToken();
        // Assert
        expect(actualRandomToken).toEqual(expectedRandomToken);
        expect(mockGetRandomValues).toHaveBeenCalledWith(expectedBuffer);
    });

    test('generateCsrfToken', () => {
        // Arrange
        const expectedCsrfToken = 'random_csrf';
        const expectedGenerateCsrfOptions = {
            expiresIn: USER_SESSION_TTL_MILLISECONDS,
            encoding: 'base64url',
            algorithm: 'sha256',
        };
        const tokenProvider: TokenProvider = createTokenProvider(expectedTokenPorviderDeps);
        mockGenerate.mockReturnValue(expectedCsrfToken);
        // Act
        const actualCsrfToken = tokenProvider.generateCsrfToken();
        // Arrange
        expect(actualCsrfToken).toEqual(expectedCsrfToken);
        expect(mockGenerate).toHaveBeenCalledWith(CSRF_SECRET, expectedGenerateCsrfOptions);
    });

    test('verifyCsrfToken', () => {
        // Arrange
        const expectedCsrfToken = 'random_csrf';
        const expectedIsValid = true;
        const expectedVerifyCsrfOptions = {
            secret: CSRF_SECRET,
            maxAge: USER_SESSION_TTL_MILLISECONDS,
            encoding: 'base64url',
            algorithm: 'sha256',
        };
        const tokenProvider: TokenProvider = createTokenProvider(expectedTokenPorviderDeps);
        mockVerify.mockReturnValue(expectedIsValid);
        // Act
        const actualIsValid = tokenProvider.verifyCsrfToken(expectedCsrfToken);
        // Assert
        expect(actualIsValid).toEqual(expectedIsValid);
        expect(mockVerify).toHaveBeenCalledWith(expectedCsrfToken, expectedVerifyCsrfOptions);
    });
});
