import { afterEach, describe, expect, mock, test } from 'bun:test';
import { createCsrfController, type CsrfController, type CsrfControllerDeps } from './csrf';
import type { SessionRepository } from '../persistence/session.kv';
import type { CsrfRepository } from '../persistence/csrf.kv';
import type { BunRequest, CookieMap } from 'bun';
import type { UserSession } from '../domain/session.entity';
import { USER_SESSION_COOKIE_NAME } from '../constants';
import { SessionNotFoundError } from '../domain/errors/SessionNotFoundError';
import type { CsrfProvider } from '../providers/csrf';

describe('csrf', () => {
    const mockCreateUserSession = mock();
    const mockDeleteUserSesssion = mock();
    const mockGetUserSession = mock();
    const mockCheckAnonymousCsrf = mock();
    const mockCreateAnonymousCsrf = mock();
    const mockDeleteAnonymousCsrf = mock();
    const mockGetCookie = mock();
    const mockDeleteCookie = mock();
    const mockGenerateCsrf = mock();
    const mockVerifyCsrf = mock();
    const mockSessionRepository: SessionRepository = {
        createUserSession: mockCreateUserSession,
        deleteUserSession: mockDeleteUserSesssion,
        getUserSession: mockGetUserSession,
    };
    const mockCsrfRepository: CsrfRepository = {
        checkAnonymousCsrf: mockCheckAnonymousCsrf,
        createAnonymousCsrf: mockCreateAnonymousCsrf,
        deleteAnonymousCsrf: mockDeleteAnonymousCsrf,
    };
    const mockCsrfProvider: CsrfProvider = {
        generate: mockGenerateCsrf,
        verify: mockVerifyCsrf,
    };
    const expectedCsrfControllerDeps: CsrfControllerDeps = {
        sessionRepository: mockSessionRepository,
        csrfRepository: mockCsrfRepository,
        csrfProvider: mockCsrfProvider,
    };
    const mockCookies = {
        get: mockGetCookie,
        delete: mockDeleteCookie,
    } satisfies Partial<CookieMap>;

    afterEach(() => {
        mockCreateUserSession.mockReset();
        mockDeleteUserSesssion.mockReset();
        mockGetUserSession.mockReset();
        mockCheckAnonymousCsrf.mockReset();
        mockCreateAnonymousCsrf.mockReset();
        mockDeleteAnonymousCsrf.mockReset();
        mockGetCookie.mockReset();
        mockDeleteCookie.mockReset();
        mockGenerateCsrf.mockReset();
        mockVerifyCsrf.mockReset();
    });

    test('createCsrfController', () => {
        // Arrange
        const expectedCsrfController: CsrfController = {
            getToken: expect.any(Function),
        };
        // Act
        const actualCsrfController: CsrfController = createCsrfController(
            expectedCsrfControllerDeps,
        );
        // Assert
        expect(actualCsrfController).toEqual(expectedCsrfController);
    });

    describe('getToken', () => {
        const expectedCsrfToken = 'csrf_token';
        const expectedSessionToken = 'random_value';
        const expectedUserId = '019dc524-bf0c-773c-93b1-c8aab19be3b2';
        const expectedUsername = 'test';
        const expectedUserSession: UserSession = {
            token: expectedSessionToken,
            csrfToken: expectedCsrfToken,
            userId: expectedUserId,
            username: expectedUsername,
        };
        const expectedRequest = {
            cookies: mockCookies as unknown as CookieMap,
        } satisfies Partial<BunRequest>;

        test('return csrf from session if session exists', async () => {
            // Arrange
            const expectedResponseJson = {
                csrfToken: expectedCsrfToken,
            };
            mockGetCookie.mockReturnValue(expectedSessionToken);
            mockGetUserSession.mockResolvedValue(expectedUserSession);
            const csrfController: CsrfController = createCsrfController(expectedCsrfControllerDeps);
            // Act
            const actualResponse = await csrfController.getToken(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(mockGetCookie).toHaveBeenCalledWith(USER_SESSION_COOKIE_NAME);
            expect(mockGetUserSession).toHaveBeenCalledWith(expectedSessionToken);
        });

        test('return anonymous csfr and delete cookie if cookie exists but session is invalid', async () => {
            // Arrange
            const expectedResponseJson = {
                csrfToken: expectedCsrfToken,
            };
            mockGetCookie.mockReturnValue(expectedSessionToken);
            mockGetUserSession.mockImplementation(() => {
                throw new SessionNotFoundError();
            });
            mockGenerateCsrf.mockReturnValue(expectedCsrfToken);
            const csrfController: CsrfController = createCsrfController(expectedCsrfControllerDeps);
            // Act
            const actualResponse = await csrfController.getToken(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(mockGetCookie).toHaveBeenCalledWith(USER_SESSION_COOKIE_NAME);
            expect(mockGenerateCsrf).toHaveBeenCalled();
            expect(mockCreateAnonymousCsrf).toHaveBeenCalledWith(expectedCsrfToken);
            expect(mockDeleteCookie).toHaveBeenCalledWith(USER_SESSION_COOKIE_NAME);
        });

        test('return anonymous csrf token if cookes does not exist', async () => {
            // Arrange
            const expectedResponseJson = {
                csrfToken: expectedCsrfToken,
            };
            mockGenerateCsrf.mockReturnValue(expectedCsrfToken);
            mockGetUserSession.mockImplementation(() => {
                throw new SessionNotFoundError();
            });
            const csrfController: CsrfController = createCsrfController(expectedCsrfControllerDeps);
            // Act
            const actualResponse = await csrfController.getToken(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(mockGetCookie).toHaveBeenCalledWith(USER_SESSION_COOKIE_NAME);
            expect(mockGenerateCsrf).toHaveBeenCalled();
            expect(mockCreateAnonymousCsrf).toHaveBeenCalledWith(expectedCsrfToken);
        });

        test('throws an error if getUserSession throws an unexpected error', async () => {
            // Arrange
            const expectedUnexpectedError = Symbol('UNEXPECTED');
            mockGetCookie.mockReturnValue(expectedSessionToken);
            mockGetUserSession.mockRejectedValue(expectedUnexpectedError);
            const csrfController: CsrfController = createCsrfController(expectedCsrfControllerDeps);
            try {
                // Act
                await csrfController.getToken(expectedRequest as unknown as BunRequest);
            } catch (actualError) {
                // Assert
                expect(actualError).toEqual(expectedUnexpectedError);
            }
        });
    });
});
