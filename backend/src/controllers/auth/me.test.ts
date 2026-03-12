import { describe, expect, test } from 'bun:test';
import type { BunRequest } from 'bun';
import { me } from './me';
import type { RequestContext } from '../types';
import type { UserSession } from '@/src/domain/session.entity';

describe('me', () => {
    test('returns username from context', async () => {
        // Arrange
        const expectedToken = 'token';
        const expectedUserId = '019cb419-2d24-727d-b66a-cf6390891464';
        const expectedUsername = 'username';
        const expectedResponseJson = { username: expectedUsername };
        const expectedUserSession: UserSession = {
            token: expectedToken,
            userId: expectedUserId,
            username: expectedUsername,
        };
        const expectedContext: RequestContext = {
            user: expectedUserSession,
        };
        const expectedRequest = {} satisfies Partial<BunRequest>;
        const expectedResponse: Response = Response.json(expectedResponseJson);
        // Act
        const actualResponse: Response = me(
            expectedRequest as unknown as BunRequest,
            expectedContext,
        );
        // Assert
        expect(await actualResponse.json()).toEqual(expectedResponseJson);
        expect(actualResponse.status).toEqual(expectedResponse.status);
    });
});
