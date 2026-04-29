import type { UserRepository } from '@/src/persistence/user.db';
import { afterEach, describe, expect, mock, test } from 'bun:test';
import { createLoginController, type LoginController, type LoginControllerDeps } from './login';
import type { SessionRepository } from '@/src/persistence/session.kv';
import type { BunRequest, CookieInit, CookieMap } from 'bun';
import { USER_SESSION_TTL_SECONDS, USER_SESSION_COOKIE_NAME, CSRF_HEADER } from '@/src/constants';
import { createUserFixture } from '@/tests/fixtures/users';
import { constants } from 'node:http2';
import type { UserSession } from '@/src/domain/session.entity';
import type { TokenProvider } from '@/src/providers/token';
import type { CsrfRepository } from '@/src/persistence/csrf.kv';

describe('login', () => {
    const mockSetCookies = mock();
    const mockVerifyHash = mock();
    const mockAddUser = mock();
    const mockGetUserByUsername = mock();
    const mockCreateUserSession = mock();
    const mockDeleteUserSession = mock();
    const mockGetUserSession = mock();
    const mockGetRandomToken = mock();
    const mockGenerateCsrfToken = mock();
    const mockVerifyCsrfToken = mock();
    const mockCreateAnonymousCsrf = mock();
    const mockDeleteAnonymousCsrf = mock();
    const mockVerifyAnonymousCsrf = mock();
    const mockGetHeader = mock();
    const mockUserRepository: UserRepository = {
        addUser: mockAddUser,
        getUserByUsername: mockGetUserByUsername,
    };
    const mockSessionRepository: SessionRepository = {
        createUserSession: mockCreateUserSession,
        getUserSession: mockGetUserSession,
        deleteUserSession: mockDeleteUserSession,
    };
    const mockCsrfRepository: CsrfRepository = {
        createAnonymousCsrf: mockCreateAnonymousCsrf,
        deleteAnonymousCsrf: mockDeleteAnonymousCsrf,
        verifyAnonymousCsrf: mockVerifyAnonymousCsrf,
    };
    const mockCookies = {
        set: mockSetCookies,
    } satisfies Partial<CookieMap>;
    const mockHeaders = {
        get: mockGetHeader,
    } satisfies Partial<Headers>;
    const mockTokenProvider: TokenProvider = {
        generateRandomToken: mockGetRandomToken,
        generateCsrfToken: mockGenerateCsrfToken,
        verifyCsrfToken: mockVerifyCsrfToken,
    };

    const expectedLoginControllerDeps: LoginControllerDeps = {
        userRepository: mockUserRepository,
        sessionRepository: mockSessionRepository,
        csrfRepository: mockCsrfRepository,
        verifyHash: mockVerifyHash,
        tokenProvider: mockTokenProvider,
    };

    afterEach(() => {
        mockSetCookies.mockReset();
        mockVerifyHash.mockReset();
        mockAddUser.mockReset();
        mockGetUserByUsername.mockReset();
        mockCreateUserSession.mockReset();
        mockGetUserSession.mockReset();
        mockDeleteUserSession.mockReset();
        mockGetRandomToken.mockReset();
        mockGenerateCsrfToken.mockReset();
        mockVerifyCsrfToken.mockReset();
        mockCreateAnonymousCsrf.mockReset();
        mockDeleteAnonymousCsrf.mockReset();
        mockVerifyAnonymousCsrf.mockReset();
    });

    test('createLoginController', () => {
        // Arrange
        // Act
        const actualLoginController: LoginController = createLoginController(
            expectedLoginControllerDeps,
        );
        // Assert
        expect(actualLoginController.post).toBeFunction();
    });

    describe('post controller', () => {
        test('passwords match', async () => {
            // Arrange
            const expectedCsrfToken = 'csrf_token';
            const expectedAnonymousCsrfToken = 'anonymous_csrf';
            const expectedResponseJson = { ok: true, csrfToken: expectedCsrfToken };
            const expectedToken = 'random_token';
            const expectedUsername = 'username';
            const expectedPassword = 'password';
            const expectedHashedPassword = 'hashed_password';
            const [expectedUserEntity] = createUserFixture(
                expectedUsername,
                expectedHashedPassword,
            );
            const expectedResponse: Response = Response.json(expectedResponseJson);
            const expectedRequestJson = { username: expectedUsername, password: expectedPassword };
            const expectedRequest = {
                json: mock().mockResolvedValue(expectedRequestJson),
                cookies: mockCookies as unknown as CookieMap,
                headers: mockHeaders as unknown as Headers,
            } satisfies Partial<BunRequest>;
            const expectedCookieInit: CookieInit = {
                maxAge: USER_SESSION_TTL_SECONDS,
                httpOnly: true,
                secure: true,
                path: '/',
            };
            const expectedUserSessionInsert: UserSession = {
                token: expectedToken,
                username: expectedUsername,
                userId: expectedUserEntity.id,
                csrfToken: expectedCsrfToken,
            };
            mockVerifyHash.mockResolvedValue(true);
            mockGetUserByUsername.mockResolvedValue(expectedUserEntity);
            mockGetRandomToken.mockReturnValue(expectedToken);
            mockGenerateCsrfToken.mockReturnValue(expectedCsrfToken);
            mockGetHeader.mockReturnValue(expectedAnonymousCsrfToken);

            const loginController: LoginController = createLoginController(
                expectedLoginControllerDeps,
            );
            // Act
            const actualResponse: Response = await loginController.post(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(expectedRequest.json).toHaveBeenCalled();
            expect(mockVerifyHash).toHaveBeenCalledWith(expectedPassword, expectedHashedPassword);
            expect(mockGetUserByUsername).toHaveBeenCalledWith(expectedUsername);
            expect(mockGetRandomToken).toHaveBeenCalled();
            expect(mockGenerateCsrfToken).toHaveBeenCalled();
            expect(mockCreateUserSession).toHaveBeenCalledWith(expectedUserSessionInsert);
            expect(mockSetCookies).toHaveBeenCalledWith(
                USER_SESSION_COOKIE_NAME,
                expectedToken,
                expectedCookieInit,
            );
            expect(mockGetHeader).toHaveBeenCalledWith(CSRF_HEADER);
            expect(mockDeleteAnonymousCsrf).toHaveBeenCalledWith(expectedAnonymousCsrfToken);
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
            expect(actualResponse.status).toEqual(expectedResponse.status);
        });

        test('passwords do not match', async () => {
            // Arrange
            const expectedResponseJson = { ok: false };
            const expectedUsername = 'username';
            const expectedPassword = 'password';
            const expectedHashedPassword = 'hashed_password';
            const [, expectedUserEntity] = createUserFixture(
                expectedUsername,
                expectedHashedPassword,
            );
            const expectedResponse: Response = Response.json(expectedResponseJson, {
                status: constants.HTTP_STATUS_UNAUTHORIZED,
            });
            const expectedRequestJson = { username: expectedUsername, password: expectedPassword };
            const expectedRequest = {
                json: mock().mockResolvedValue(expectedRequestJson),
                cookies: mockCookies as unknown as CookieMap,
            } satisfies Partial<BunRequest>;
            mockVerifyHash.mockResolvedValue(false);
            mockGetUserByUsername.mockResolvedValue(expectedUserEntity);

            const loginController: LoginController = createLoginController(
                expectedLoginControllerDeps,
            );
            // Act
            const actualResponse: Response = await loginController.post(
                expectedRequest as unknown as BunRequest,
            );
            // Assert
            expect(expectedRequest.json).toHaveBeenCalled();
            expect(mockVerifyHash).toHaveBeenCalledWith(expectedPassword, expectedHashedPassword);
            expect(mockGetUserByUsername).toHaveBeenCalledWith(expectedUsername);
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
            expect(actualResponse.status).toEqual(expectedResponse.status);
        });
    });
});
