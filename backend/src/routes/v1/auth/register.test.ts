import { describe, test, expect, mock } from 'bun:test';
import register from './register.ts';

mock.module('../../../controllers/auth/register.ts', () => ({}))

describe('register', () => {
    test('/api/v1/register', async () => {
        // Arrange
        const mockRegister = await import('../../../controllers/auth/register.ts');
        const expectedMethod = 'POST';
        const expectedUrl = '/api/v1/register';
        const expectedRoute = { [expectedUrl]: { [expectedMethod]: mockRegister.post } };
        // Act
        const actualRoute = register;
        // Assert
        expect(actualRoute).toEqual(expectedRoute);
    });
});
