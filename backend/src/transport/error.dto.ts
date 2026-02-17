import { VALIDATION_ERROR_CODE, UNEXPECTED_ERROR_CODE, SYNTAX_ERROR_CODE } from '$/constants';

type ErrorCode =
    | typeof VALIDATION_ERROR_CODE
    | typeof UNEXPECTED_ERROR_CODE
    | typeof SYNTAX_ERROR_CODE;

export type ErrorResponse = {
    code: ErrorCode,
    error: string | object,
}
