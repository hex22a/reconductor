import type { ValidationError } from '@/src/transport/error.dto';
import type { GraphQlContext } from '@/src/transport/graphql.context';
import type { ScanDto } from '@/src/transport/scan.dto';
import { afterEach, describe, expect, mock, test } from 'bun:test';
import type { CreateScanArgs } from '../resolvers/scan';
import type { CreateEntityPayload } from '@/src/transport/payload.dto';
import type { Edge } from '@/src/transport/edge.dto';
import { withValidation } from './mutation';
import { ZodIssueCode } from 'zod/v3';
import type { $ZodIssue } from 'zod/v4/core';
import { ZodError } from 'zod';
import { Z_SCAN_PROJECT_ID_ERROR_MESSAGE, Z_SCAN_TARGET_ERROR_MESSAGE } from '@/src/constants';

describe('mutation', () => {
    const expectedParent = null;
    const expectedUserId = '019d3461-29dd-7ed8-9904-6f6441e834ad';
    const expectedScanId = '019d3461-af6b-761f-91e0-ebe5d64f47b5';
    const expectedProjectId = '019d3462-37cd-78cf-8e1a-09ecbdc92f27';
    const expectedCursor = 'cursor';

    const mockMutation = mock();

    afterEach(() => {
        mockMutation.mockReset();
    });

    describe('withFieldValidation', () => {
        test('no errors', async () => {
            // Arrange
            const expectedTarget = '192.168.50.0/16';
            const expectedValidationErrors: Array<ValidationError> = [];
            const expectedScan: ScanDto = {
                id: expectedScanId,
                created_at: expect.any(Date),
                target: expectedTarget,
            };
            const expectedArgs: CreateScanArgs = {
                input: {
                    target: expectedTarget,
                    projectId: expectedProjectId,
                },
            };
            const expectedContext: GraphQlContext = {
                user: { id: expectedUserId },
            };
            const expectedEdge: Edge<ScanDto> = {
                node: expectedScan,
                cursor: expectedCursor,
            };
            const expectedResponse: CreateEntityPayload<Edge<ScanDto>> = {
                edge: expectedEdge,
                errors: expectedValidationErrors,
            };
            mockMutation.mockResolvedValue(expectedResponse);
            const decoratedMutation = withValidation(mockMutation);
            // Act
            const actualResponse = await decoratedMutation(
                expectedParent,
                expectedArgs,
                expectedContext,
            );
            // Assert
            expect(actualResponse).toEqual(expectedResponse);
        });

        test('mutation throws a zod validation error', async () => {
            // Arrange
            const expectedTarget = '192.168.50.0/16';
            const expectedFirstIssuePath = 'target';
            const expectedSecondIssuePath = 'projectId';
            const expectedZodIssues: $ZodIssue[] = [
                {
                    code: ZodIssueCode.invalid_type,
                    path: [expectedFirstIssuePath],
                    message: Z_SCAN_TARGET_ERROR_MESSAGE,
                    expected: 'string',
                },
                {
                    code: ZodIssueCode.invalid_type,
                    path: [expectedSecondIssuePath],
                    message: Z_SCAN_PROJECT_ID_ERROR_MESSAGE,
                    expected: 'string',
                },
            ];
            const expectedValidationErrors: Array<ValidationError> = [
                {
                    field: expectedFirstIssuePath,
                    message: Z_SCAN_TARGET_ERROR_MESSAGE,
                },
                {
                    field: expectedSecondIssuePath,
                    message: Z_SCAN_PROJECT_ID_ERROR_MESSAGE,
                },
            ];
            const expectedArgs: CreateScanArgs = {
                input: {
                    target: expectedTarget,
                    projectId: expectedProjectId,
                },
            };
            const expectedContext: GraphQlContext = {
                user: { id: expectedUserId },
            };
            const expectedResponse: CreateEntityPayload<Edge<ScanDto>> = {
                errors: expectedValidationErrors,
            };
            mockMutation.mockRejectedValue(new ZodError(expectedZodIssues));
            const decoratedMutation = withValidation(mockMutation);
            // Act
            const actualResponse = await decoratedMutation(
                expectedParent,
                expectedArgs,
                expectedContext,
            );
            // Assert
            expect(actualResponse).toEqual(expectedResponse);
        });
    });
});
