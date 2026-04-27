import type { BunRequest } from 'bun';
import { afterEach, describe, expect, mock, test } from 'bun:test';
import { withErrorHandling } from './withErrorHandling';
import type { ErrorResponse } from '@/src/transport/error.dto';
import {
    DATABASE_ERROR_CODE,
    SYNTAX_ERROR_CODE,
    UNEXPECTED_ERROR_CODE,
    VALIDATION_ERROR_CODE,
} from '$/constants';
import {
    DATABASE_ERROR_MESSAGE,
    UNEXPECTED_END_OF_JSON_ERROR_MESSAGE,
    UNEXPECTED_ERROR_MESSAGE,
    Z_PASSWORD_STRING_ERROR_MESSAGE,
    Z_USERNAME_STRING_ERROR_MESSAGE,
} from '@/src/constants';
import { constants } from 'node:http2';
import { ZodError } from 'zod';
import { ZodIssueCode } from 'zod/v3';
import type { $ZodIssue } from 'zod/v4/core';

describe('withErrorHandling', () => {
    const mockController = mock();

    afterEach(() => {
        mockController.mockReset();
    });

    test('no errors', async () => {
        // Arrange
        const expectedResponseJson = { respose: 'some valid response' };
        const expectedResponse: Response = Response.json(expectedResponseJson);

        const expectedRequest = {} satisfies Partial<BunRequest>;
        mockController.mockResolvedValue(expectedResponse);
        const decoratedController = withErrorHandling(mockController);

        // Act
        const actualResponse: Response = await decoratedController(
            expectedRequest as unknown as BunRequest,
        );

        // Assert
        expect(mockController).toHaveBeenCalledWith(expectedRequest);
        expect(await actualResponse.json()).toEqual(expectedResponseJson);
        expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
        expect(actualResponse.status).toEqual(expectedResponse.status);
    });

    test('returns 500 for unexpected error', async () => {
        // Arrange
        const expectedResponseJson: ErrorResponse = {
            code: UNEXPECTED_ERROR_CODE,
            error: UNEXPECTED_ERROR_MESSAGE,
        };
        const expectedResponseInit: ResponseInit = {
            status: constants.HTTP_STATUS_INTERNAL_SERVER_ERROR,
        };
        const expectedResponse: Response = Response.json(
            expectedResponseJson,
            expectedResponseInit,
        );

        const expectedRequest = {} satisfies Partial<BunRequest>;

        mockController.mockRejectedValue(Symbol('UNEXPECTED'));
        const decoratedController = withErrorHandling(mockController);

        // Act
        const actualResponse: Response = await decoratedController(
            expectedRequest as unknown as BunRequest,
        );

        // Assert
        expect(mockController).toHaveBeenCalledWith(expectedRequest);
        expect(await actualResponse.json()).toEqual(expectedResponseJson);
        expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
        expect(actualResponse.status).toEqual(expectedResponse.status);
    });

    test('returns 400 when no JSON provided', async () => {
        // Arrange
        const expectedResponseJson: ErrorResponse = {
            code: SYNTAX_ERROR_CODE,
            error: UNEXPECTED_END_OF_JSON_ERROR_MESSAGE,
        };
        const expectedResponseInit: ResponseInit = {
            status: constants.HTTP_STATUS_BAD_REQUEST,
        };
        const expectedResponse: Response = Response.json(
            expectedResponseJson,
            expectedResponseInit,
        );

        const expectedRequest = {} satisfies Partial<BunRequest>;
        mockController.mockRejectedValue(new SyntaxError(UNEXPECTED_END_OF_JSON_ERROR_MESSAGE));
        const decoratedController = withErrorHandling(mockController);

        // Act
        const actualResponse: Response = await decoratedController(
            expectedRequest as unknown as BunRequest,
        );

        // Assert
        expect(mockController).toHaveBeenCalledWith(expectedRequest);
        expect(await actualResponse.json()).toEqual(expectedResponseJson);
        expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
        expect(actualResponse.status).toEqual(expectedResponse.status);
    });

    test('returns 422 when json does not match the schema', async () => {
        // Arrange
        const expectedFirstIssuePath = 'username';
        const expectedSecondIssuePath = 'password';
        const expectedZodIssues: $ZodIssue[] = [
            {
                code: ZodIssueCode.invalid_type,
                path: [expectedFirstIssuePath],
                message: Z_USERNAME_STRING_ERROR_MESSAGE,
                expected: 'string',
            },
            {
                code: ZodIssueCode.invalid_type,
                path: [expectedSecondIssuePath],
                message: Z_PASSWORD_STRING_ERROR_MESSAGE,
                expected: 'string',
            },
        ];
        const expectedResponseJson: ErrorResponse = {
            code: VALIDATION_ERROR_CODE,
            error: {
                fieldErrors: {
                    [expectedFirstIssuePath]: [Z_USERNAME_STRING_ERROR_MESSAGE],
                    [expectedSecondIssuePath]: [Z_PASSWORD_STRING_ERROR_MESSAGE],
                },
                formErrors: [],
            },
        };
        const expectedResponseInit: ResponseInit = {
            status: constants.HTTP_STATUS_UNPROCESSABLE_ENTITY,
        };

        const expectedResponse: Response = Response.json(
            expectedResponseJson,
            expectedResponseInit,
        );

        const expectedRequest = {} satisfies Partial<BunRequest>;
        mockController.mockRejectedValue(new ZodError(expectedZodIssues));
        const decoratedController = withErrorHandling(mockController);

        // Act
        const actualResponse: Response = await decoratedController(
            expectedRequest as unknown as BunRequest,
        );
        // Assert
        expect(mockController).toHaveBeenCalledWith(expectedRequest);
        expect(await actualResponse.json()).toEqual(expectedResponseJson);
        expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
        expect(actualResponse.status).toEqual(expectedResponse.status);
    });

    test('returns 500 for database errors', async () => {
        // Arrange
        const expectedErrorMessage = 'database error';
        const expectedErrorCode = '23505';
        const expectedResponseJson: ErrorResponse = {
            code: DATABASE_ERROR_CODE,
            error: DATABASE_ERROR_MESSAGE,
        };
        const expectedResponseInit: ResponseInit = {
            status: constants.HTTP_STATUS_INTERNAL_SERVER_ERROR,
        };

        const expectedResponse: Response = Response.json(
            expectedResponseJson,
            expectedResponseInit,
        );

        const expectedRequest = {} satisfies Partial<BunRequest>;
        mockController.mockRejectedValue(
            new Bun.SQL.PostgresError(expectedErrorMessage, { code: expectedErrorCode }),
        );
        const decoratedController = withErrorHandling(mockController);

        // Act
        const actualResponse: Response = await decoratedController(
            expectedRequest as unknown as BunRequest,
        );
        // Assert
        expect(mockController).toHaveBeenCalledWith(expectedRequest);
        expect(await actualResponse.json()).toEqual(expectedResponseJson);
        expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
        expect(actualResponse.status).toEqual(expectedResponse.status);
    });
});
