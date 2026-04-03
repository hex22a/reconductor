import { CronExpressionParser } from 'cron-parser';

export type CronParserFactoryDeps = {
    CronParserProvider: typeof CronExpressionParser;
};

export type CronParser = {
    getNextRunDate: (cronSchedule: string) => Date;
};

export function createCronParser({ CronParserProvider }: CronParserFactoryDeps): CronParser {
    return {
        getNextRunDate(cronSchedule): Date {
            return CronParserProvider.parse(cronSchedule).next().toDate();
        },
    };
}
