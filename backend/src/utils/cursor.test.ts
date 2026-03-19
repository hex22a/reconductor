import { describe, expect, test } from 'bun:test';
import { decodeCursor, encodeCursor } from './cursor';

describe('cursor', () => {
    const expectedId = '019d0634-b90c-7400-99d2-253d76dfb3ea';
    const expectedCursor = 'MDE5ZDA2MzQtYjkwYy03NDAwLTk5ZDItMjUzZDc2ZGZiM2Vh';

    test('encodeCursor', () => {
        // Arrange
        // Act
        const actualCursor = encodeCursor(expectedId);
        // Assert
        expect(actualCursor).toEqual(expectedCursor);
    });

    test('decodeCursor', () => {
        // Arrange
        // Act
        const actualId = decodeCursor(expectedCursor);
        // Assert
        expect(actualId).toEqual(expectedId);
    });
});
