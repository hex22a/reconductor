import type { ValidationError } from '@/src/transport/error.dto';
import type { GraphQlContext } from '@/src/transport/graphql.context';
import type { ScanDto } from '@/src/transport/scan.dto';
import { afterEach, describe, expect, mock, test } from 'bun:test';
import type { CreateScanArgs } from '../resolvers/scan';
import type { CreateEntityPayload } from '@/src/transport/payload.dto';
import type { Edge } from '@/src/transport/edge.dto';
import { withValidation } from './mutation';

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
        test('no error', async () => {
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
    });
});
