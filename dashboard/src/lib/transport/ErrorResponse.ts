import {
    VALIDATION_ERROR_CODE,
    UNEXPECTED_ERROR_CODE,
    SYNTAX_ERROR_CODE,
    DATABASE_ERROR_CODE,
    NETWORK_ERROR_CODE,
} from '@/constants';

type ErrorCode =
    | typeof DATABASE_ERROR_CODE
    | typeof VALIDATION_ERROR_CODE
    | typeof UNEXPECTED_ERROR_CODE
    | typeof NETWORK_ERROR_CODE
    | typeof SYNTAX_ERROR_CODE;

export type ErrorResponse = {
    code: ErrorCode;
    error: string | ValidationError;
};

export type ValidationError = {
    field_errors: Map<string, [string]>;
};

export function isError(value: unknown): value is ErrorResponse {
    return value !== null && typeof value === 'object' && 'code' in value;
}
