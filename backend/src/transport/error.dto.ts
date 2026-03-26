import {
    VALIDATION_ERROR_CODE,
    UNEXPECTED_ERROR_CODE,
    SYNTAX_ERROR_CODE,
    DATABASE_ERROR_CODE,
} from '$/constants';

type ErrorCode =
    | typeof DATABASE_ERROR_CODE
    | typeof VALIDATION_ERROR_CODE
    | typeof UNEXPECTED_ERROR_CODE
    | typeof SYNTAX_ERROR_CODE;

export type ErrorResponse = {
    code: ErrorCode;
    error: string | object;
};

export type ValidationError = {
    field: string;
    message: string;
};
