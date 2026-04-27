import { createCsrfRepository, type CsrfRepository } from '@/src/persistence/csrf.kv';
import { kv } from '@/src/persistence/kv';
import { afterEach, describe, expect, it, test } from 'bun:test';

describe('csrf', () => {
    afterEach(async () => {
        await kv.send('FLUSHDB', ['ASYNC']);
    });

    test('createCsrfRepository', () => {
        // Arrange
        const expectedCsrfRepository: CsrfRepository = {
            createAnonymousCsrf: expect.any(Function),
            verifyAnonymousCsrf: expect.any(Function),
            deleteAnonymousCsrf: expect.any(Function),
        };
        // Act
        const actualCsrfRepository = createCsrfRepository({ kv });
        // Assert
        expect(actualCsrfRepository).toEqual(expectedCsrfRepository);
    });

    it('stores anonymous csrf under anonymous_csrf:{token} key', async () => {
        // Arrange
        const expectedToken = 'random_value';
        const csrfRepository = createCsrfRepository({ kv });
        // Act
        const actualIsAdded = await csrfRepository.createAnonymousCsrf(expectedToken);
        // Assert
        expect(actualIsAdded).toEqual(true);
    });

    it('returns false if token not found', async () => {
        // Arrange
        const expectedToken = 'does_not_exist';
        const csrfRepository = createCsrfRepository({ kv });
        // Act
        const actualIsFound = await csrfRepository.verifyAnonymousCsrf(expectedToken);
        // Assert
        expect(actualIsFound).toEqual(false);
    });

    it('returns true if token is found', async () => {
        // Arrange
        const expectedToken = 'random_value';
        const csrfRepository = createCsrfRepository({ kv });
        await csrfRepository.createAnonymousCsrf(expectedToken);
        // Act
        const actualIsFound = await csrfRepository.verifyAnonymousCsrf(expectedToken);
        // Assert
        expect(actualIsFound).toEqual(true);
    });

    it('deletes csrf from storage', async () => {
        const expectedToken = 'random_value';
        const csrfRepository = createCsrfRepository({ kv });
        await csrfRepository.createAnonymousCsrf(expectedToken);
        // Act
        await csrfRepository.deleteAnonymousCsrf(expectedToken);
        // Assert
        const actualIsFound = await csrfRepository.verifyAnonymousCsrf(expectedToken);
        expect(actualIsFound).toEqual(false);
    });
});
