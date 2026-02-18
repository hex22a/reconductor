import type { UserRepository } from '@/src/persistence/user.db';
import { afterEach, describe, expect, mock, test } from 'bun:test';
import { createLoginController, type LoginController } from './login';
import type { SessionRepository } from '@/src/persistence/session.kv';
import type { BunRequest, CookieInit, CookieMap } from 'bun';
import { HEADERS, USER_SESSION_COOKIE_MAX_AGE, USER_SESSION_COOKIE_NAME } from '@/src/constants';
import { createUserFixture } from '@/tests/fixtures/users';
import type { UserSessionInsert } from '@/src/domain/session.entity';
import { constants } from 'node:http2';

describe('login', () => {
    const mockSetCookies = mock();
    const mockVerifyHash = mock();
    const mockAddUser = mock();
    const mockGetUserByUsername = mock();
    const mockCreateUserSession = mock();
    const mockGetUserSession = mock();
    const mockGetRandomToken = mock();
    const mockUserRepository: UserRepository = {
        addUser: mockAddUser,
        getUserByUsername: mockGetUserByUsername,
    };
    const mockSessionRepository: SessionRepository = {
        createUserSession: mockCreateUserSession,
        getUserSession: mockGetUserSession,
    };
    const mockCookies = {
        set: mockSetCookies,
    } satisfies Partial<CookieMap>;

    afterEach(() => {
        mockSetCookies.mockReset();
        mockVerifyHash.mockReset();
        mockAddUser.mockReset();
        mockGetUserByUsername.mockReset();
        mockCreateUserSession.mockReset();
        mockGetUserSession.mockReset();
        mockGetRandomToken.mockReset();
    });

    test('createLoginController', () => {
        // Arrange
        // Act
        const actualLoginController: LoginController = createLoginController(
            mockUserRepository,
            mockSessionRepository,
            mockVerifyHash,
            mockGetRandomToken,
        );
        // Assert
        expect(actualLoginController.post).toBeFunction();
    });

    describe('post controller', () => {
        test('passwords match', async () => {
            // Arrange
            const expectedResponseJson = { ok: true };
            const expectedToken = 'random_token';
            const expectedUsername = 'username';
            const expectedPassword = 'password';
            const expectedHashedPassword = 'hashed_password';
            const [, expectedUserEntity] = createUserFixture(expectedUsername, expectedHashedPassword);
            const expectedResponse: Response = Response.json(expectedResponseJson, { headers: HEADERS });
            const expectedRequestJson = { username: expectedUsername, password: expectedPassword };
            const expectedRequest = {
                json: mock().mockResolvedValue(expectedRequestJson),
                cookies: (mockCookies as unknown as CookieMap),
            } satisfies Partial<BunRequest>;
            const expectedCookieInit: CookieInit = {
                maxAge: USER_SESSION_COOKIE_MAX_AGE,
                httpOnly: true,
                secure: true,
                path: '/',
            };
            const expectedUserSessionInsert: UserSessionInsert = {
                token: expectedToken,
                userId: expectedUserEntity.id,
            };
            mockVerifyHash.mockResolvedValue(true);
            mockGetUserByUsername.mockResolvedValue(expectedUserEntity);
            mockGetRandomToken.mockReturnValue(expectedToken);

            const loginController: LoginController = createLoginController(
                mockUserRepository,
                mockSessionRepository,
                mockVerifyHash,
                mockGetRandomToken,
            );
            // Act
            const actualResponse: Response = await loginController.post(expectedRequest as unknown as BunRequest);
            // Assert
            expect(expectedRequest.json).toHaveBeenCalled();
            expect(mockVerifyHash).toHaveBeenCalledWith(expectedPassword, expectedHashedPassword);
            expect(mockGetUserByUsername).toHaveBeenCalledWith(expectedUsername);
            expect(mockGetRandomToken).toHaveBeenCalled();
            expect(mockCreateUserSession).toHaveBeenCalledWith(expectedUserSessionInsert);
            expect(mockSetCookies).toHaveBeenCalledWith(USER_SESSION_COOKIE_NAME, expectedToken, expectedCookieInit);
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
            const [, expectedUserEntity] = createUserFixture(expectedUsername, expectedHashedPassword);
            const expectedResponse: Response = Response.json(
                expectedResponseJson,
                { headers: HEADERS, status: constants.HTTP_STATUS_UNAUTHORIZED },
            );
            const expectedRequestJson = { username: expectedUsername, password: expectedPassword };
            const expectedRequest = {
                json: mock().mockResolvedValue(expectedRequestJson),
                cookies: (mockCookies as unknown as CookieMap),
            } satisfies Partial<BunRequest>;
            mockVerifyHash.mockResolvedValue(false);
            mockGetUserByUsername.mockResolvedValue(expectedUserEntity);

            const loginController: LoginController = createLoginController(
                mockUserRepository,
                mockSessionRepository,
                mockVerifyHash,
                mockGetRandomToken,
            );
            // Act
            const actualResponse: Response = await loginController.post(expectedRequest as unknown as BunRequest);
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
