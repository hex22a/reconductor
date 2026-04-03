import { afterEach, describe, expect, mock, test } from 'bun:test';
import { createCronParser, type CronParser, type CronParserFactoryDeps } from './cron';
import { CronDate, CronExpression, CronExpressionParser } from 'cron-parser';

describe('cron', () => {
    const mockParse = mock();
    const mockNext = mock();
    const mockToDate = mock();
    const mockCronExpression: Partial<CronExpression> = {
        next: mockNext,
    };
    const mockCronDate: Partial<CronDate> = {
        toDate: mockToDate,
    };
    CronExpressionParser.parse = mockParse;
    const expectedCronParserDeps: CronParserFactoryDeps = {
        CronParserProvider: CronExpressionParser,
    };

    afterEach(() => {
        mockParse.mockReset();
        mockNext.mockReset();
        mockToDate.mockReset();
    });

    test('createCronParser', () => {
        // Arrange
        const expectedCronParser: CronParser = {
            getNextRunDate: expect.any(Function),
        };
        // Act
        const actualCronParser: CronParser = createCronParser(expectedCronParserDeps);
        // Assert
        expect(actualCronParser).toEqual(expectedCronParser);
    });

    test('getNextRunDate', () => {
        // Arrange
        const expectedCronSchedule = '10 5 * * *';
        const expectedNextRunDate = new Date(2030, 6, 10, 1, 30, 30, 10);
        mockParse.mockReturnValue(mockCronExpression);
        mockNext.mockReturnValue(mockCronDate);
        mockToDate.mockReturnValue(expectedNextRunDate);
        const cronParser: CronParser = createCronParser(expectedCronParserDeps);
        // Act
        const actualNextRunDate: Date = cronParser.getNextRunDate(expectedCronSchedule);
        // Assert
        expect(actualNextRunDate).toEqual(expectedNextRunDate);
    });
});
