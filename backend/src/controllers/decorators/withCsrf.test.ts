import { afterEach, describe, expect, it, mock, test } from 'bun:test';
import {
    createCsrfDecorators,
    type CsrfDecoratorsFactoryDeps,
    type CsrfDecorators,
} from './withCsrf';
import type { RequestContext } from '../types';
import type { BunRequest, MaybePromise } from 'bun';
import type { IHandleStrategy } from '../strategies/IHandleStrategy';
import type { ICsrfStrategy } from '../strategies/ICsrfStrategy';
import type { CsrfProvider } from '@/src/providers/csrf';
import type { UserSession } from '@/src/domain/session.entity';
import { FORBIDDEN_ERROR_MESSAGE } from '@/src/constants';
import { constants } from 'node:http2';

describe('csrf', () => {
    const mockGenerateCsrf = mock();
    const mockVerifyCsrf = mock();

    const mockCsrfProvider: CsrfProvider = {
        generate: mockGenerateCsrf,
        verify: mockVerifyCsrf,
    };
    class MockCsrfStrategy implements ICsrfStrategy {
        csrfProvider: CsrfProvider = mockCsrfProvider;
        verifyCsrfToken(): MaybePromise<boolean> {
            throw new Error('Called mock method');
        }
    }
    class MockHandleStrategy implements IHandleStrategy<RequestContext> {
        handle(): MaybePromise<Response> {
            throw new Error('Method not implemented.');
        }
    }
    const mockHandle = mock();
    const mockVerifyCsrfToken = mock();
    const mockHandleStrategy = new MockHandleStrategy();
    const mockCsrfStrategy = new MockCsrfStrategy();
    mockHandleStrategy.handle = mockHandle;
    mockCsrfStrategy.verifyCsrfToken = mockVerifyCsrfToken;

    const mockCsrfDecoratorsDeps: CsrfDecoratorsFactoryDeps<RequestContext> = {
        csrfStrategy: mockCsrfStrategy,
        handleStrategy: mockHandleStrategy,
    };

    const mockController = mock();

    afterEach(() => {
        mockController.mockReset();
        mockHandle.mockReset();
        mockVerifyCsrfToken.mockReset();
    });

    test('createCsrfDecorators', () => {
        // Arrange
        const expectedCsrfDecorators: CsrfDecorators<RequestContext> = {
            withCsrf: expect.any(Function),
        };
        // Act
        const actualCsrfDecorators: CsrfDecorators<RequestContext> =
            createCsrfDecorators(mockCsrfDecoratorsDeps);
        // Assert
        expect(actualCsrfDecorators).toEqual(expectedCsrfDecorators);
    });

    describe('withCsrf', () => {
        const expectedRequest = {} satisfies Partial<BunRequest>;

        it('calls handler if csrf is valid', async () => {
            // Arrange
            const expectedUserId = '019c94f3-4826-7c85-a663-98055fe5cba4';
            const expectedUsername = 'username';
            const expectedToken = 'token';
            const expectedCsrfToken = 'csrf_token';
            const expectedUserSession: UserSession = {
                token: expectedToken,
                userId: expectedUserId,
                username: expectedUsername,
                csrfToken: expectedCsrfToken,
            };
            const expectedContext: RequestContext = {
                user: expectedUserSession,
            };
            const expectedResponseJson = { respose: 'some valid response' };
            const expectedResponse: Response = Response.json(expectedResponseJson);
            const csrfDecorators: CsrfDecorators<RequestContext> =
                createCsrfDecorators(mockCsrfDecoratorsDeps);
            const decoratedController = csrfDecorators.withCsrf(mockController);
            mockHandle.mockResolvedValue(expectedResponse);
            mockVerifyCsrfToken.mockReturnValue(true);

            // Act
            const actualResponse: Response = await decoratedController(
                expectedRequest as unknown as BunRequest,
                expectedContext,
            );
            // Assert
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(actualResponse.status).toEqual(expectedResponse.status);
        });

        it('returns 403 if csrf is invalid', async () => {
            // Arrange
            const expectedUserId = '019c94f3-4826-7c85-a663-98055fe5cba4';
            const expectedUsername = 'username';
            const expectedToken = 'token';
            const expectedCsrfToken = 'csrf_token';
            const expectedUserSession: UserSession = {
                token: expectedToken,
                userId: expectedUserId,
                username: expectedUsername,
                csrfToken: expectedCsrfToken,
            };
            const expectedContext: RequestContext = {
                user: expectedUserSession,
            };
            const expectedResponseJson = { error: FORBIDDEN_ERROR_MESSAGE };
            const csrfDecorators: CsrfDecorators<RequestContext> =
                createCsrfDecorators(mockCsrfDecoratorsDeps);
            const decoratedController = csrfDecorators.withCsrf(mockController);
            mockVerifyCsrfToken.mockReturnValue(false);

            // Act
            const actualResponse: Response = await decoratedController(
                expectedRequest as unknown as BunRequest,
                expectedContext,
            );
            // Assert
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(actualResponse.status).toEqual(constants.HTTP_STATUS_FORBIDDEN);
        });
    });
});
