import { CronExpressionParser } from 'cron-parser';

export type CronParserFactoryDeps = {
    CronParser: typeof CronExpressionParser;
};

export type CronParser = {
    getNextRunDate: (cronSchedule: string) => Date;
};

export function createCronParser({ CronParser: Parser }: CronParserFactoryDeps): CronParser {
    return {
        getNextRunDate(cronSchedule): Date {
            return Parser.parse(cronSchedule).next().toDate();
        },
    };
}
