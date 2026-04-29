import { describe, expect, mock, test } from 'bun:test';
import { AnonymousCsrfStrategy, type AnonymousCsrfStrategyDeps } from './AnonymousCsrfStrategy';
import type { CsrfRepository } from '@/src/persistence/csrf.kv';
import type { BunRequest } from 'bun';
import { CSRF_HEADER } from '@/src/constants';
import type { TokenProvider } from '@/src/providers/token';

describe('AnonymousCsrfStrategy', () => {
    const expectedCsrfToken = 'csrf_token';
    const mockVerifyAnonymousCsrf = mock();
    const mockCreateAnonymousCsrf = mock();
    const mockDeleteAnonymousCsrf = mock();
    const mockGenerateCsrf = mock();
    const mockVerifyCsrf = mock();
    const mockGetHeader = mock();
    const mockGenerateRandomToken = mock();

    const mockCsrfRepository: CsrfRepository = {
        verifyAnonymousCsrf: mockVerifyAnonymousCsrf,
        createAnonymousCsrf: mockCreateAnonymousCsrf,
        deleteAnonymousCsrf: mockDeleteAnonymousCsrf,
    };
    const mockCsrfProvider: TokenProvider = {
        generateCsrfToken: mockGenerateCsrf,
        generateRandomToken: mockGenerateRandomToken,
        verifyCsrfToken: mockVerifyCsrf,
    };
    const mockAnonymousCsrfStrategyDeps: AnonymousCsrfStrategyDeps = {
        tokenProvider: mockCsrfProvider,
        csrfRepository: mockCsrfRepository,
    };
    const mockHeaders = {
        get: mockGetHeader,
    } satisfies Partial<Headers>;
    const expectedRequest = {
        headers: mockHeaders as unknown as Headers,
    } satisfies Partial<BunRequest>;

    test('constructor', () => {
        // Arrange
        // Act
        const actualAnonymousCsrfStrategy = new AnonymousCsrfStrategy(
            mockAnonymousCsrfStrategyDeps,
        );
        // Assert
        expect(actualAnonymousCsrfStrategy.verifyCsrfToken).toBeFunction();
        expect(actualAnonymousCsrfStrategy.tokenProvider).toEqual(mockCsrfProvider);
        expect(actualAnonymousCsrfStrategy.csrfRepository).toEqual(mockCsrfRepository);
    });

    describe('verifyCsrfToken', () => {
        test('return true if csrf is valid and found in repository', async () => {
            // Arrange
            mockGetHeader.mockReturnValue(expectedCsrfToken);
            mockVerifyCsrf.mockReturnValue(true);
            mockVerifyAnonymousCsrf.mockResolvedValue(true);
            const anonymousCsrfStrategy = new AnonymousCsrfStrategy(mockAnonymousCsrfStrategyDeps);
            // Act
            const actualIsValid = await anonymousCsrfStrategy.verifyCsrfToken(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(actualIsValid).toBeTrue();
            expect(mockGetHeader).toHaveBeenCalledWith(CSRF_HEADER);
            expect(mockVerifyCsrf).toHaveBeenCalledWith(expectedCsrfToken);
            expect(mockVerifyAnonymousCsrf).toHaveBeenCalledWith(expectedCsrfToken);
        });

        test('return false if csrf is invalid', async () => {
            // Arrange
            mockGetHeader.mockReturnValue(expectedCsrfToken);
            mockVerifyCsrf.mockReturnValue(false);
            const anonymousCsrfStrategy = new AnonymousCsrfStrategy(mockAnonymousCsrfStrategyDeps);
            // Act
            const actualIsValid = await anonymousCsrfStrategy.verifyCsrfToken(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(actualIsValid).toBeFalse();
            expect(mockGetHeader).toHaveBeenCalledWith(CSRF_HEADER);
            expect(mockVerifyCsrf).toHaveBeenCalledWith(expectedCsrfToken);
            expect(mockVerifyAnonymousCsrf).toHaveBeenCalledWith(expectedCsrfToken);
        });

        test('return false if csrf is valid but not found in repository', async () => {
            // Arrange
            mockGetHeader.mockReturnValue(expectedCsrfToken);
            mockVerifyCsrf.mockReturnValue(true);
            mockVerifyAnonymousCsrf.mockResolvedValue(false);
            const anonymousCsrfStrategy = new AnonymousCsrfStrategy(mockAnonymousCsrfStrategyDeps);
            // Act
            const actualIsValid = await anonymousCsrfStrategy.verifyCsrfToken(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(actualIsValid).toBeFalse();
            expect(mockGetHeader).toHaveBeenCalledWith(CSRF_HEADER);
            expect(mockVerifyCsrf).toHaveBeenCalledWith(expectedCsrfToken);
            expect(mockVerifyAnonymousCsrf).toHaveBeenCalledWith(expectedCsrfToken);
        });

        test('return false if csrf is not found', async () => {
            // Arrange
            mockGetHeader.mockReturnValue(null);
            mockVerifyCsrf.mockReturnValue(true);
            mockVerifyAnonymousCsrf.mockResolvedValue(false);
            const anonymousCsrfStrategy = new AnonymousCsrfStrategy(mockAnonymousCsrfStrategyDeps);
            // Act
            const actualIsValid = await anonymousCsrfStrategy.verifyCsrfToken(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(actualIsValid).toBeFalse();
            expect(mockGetHeader).toHaveBeenCalledWith(CSRF_HEADER);
            expect(mockVerifyCsrf).toHaveBeenCalledWith(expectedCsrfToken);
            expect(mockVerifyAnonymousCsrf).toHaveBeenCalledWith(expectedCsrfToken);
        });
    });
});
