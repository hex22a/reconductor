import type { BunRequest } from 'bun';
import { constants } from 'node:http2';
import { describe, test, expect, mock } from 'bun:test';
import { HEADERS, UNEXPECTED_END_OF_JSON_ERROR_MESSAGE, UNEXPECTED_ERROR_MESSAGE } from '../../constants';
import { createRegisterController, type RegisterController } from './register';
import type { UserDto } from '@/src/transport/user.dto';
import { createUserFixture } from '@/tests/fixtures/users';
import { afterEach } from 'node:test';

describe('register', () => {
    const mockHash = mock();
    const mockAddUser = mock();
    const mockUserRepository = {
        addUser: mockAddUser,
    };

    afterEach(() => {
        mockHash.mockReset();
        mockAddUser.mockReset();
    })

    test('createRegisterController', () => {
        // Arrange
        // Act
        const actualRegisterController: RegisterController = createRegisterController(mockUserRepository, mockHash);
        // Assert
        expect(actualRegisterController.post).toBeFunction();

    });

    describe('post controller', () => {
        test('returns 500 for unexpected error', async () => {
            // Arrange
            const expectedResponseJson = {
                error: UNEXPECTED_ERROR_MESSAGE,
            };
            const expectedResponseInit: ResponseInit = {
                headers: HEADERS,
                status: constants.HTTP_STATUS_INTERNAL_SERVER_ERROR,
            };
            const expectedResponse: Response = Response.json(expectedResponseJson, expectedResponseInit);

            const expectedRequest = {
                json: mock().mockRejectedValue(Symbol('UNEXPECTED')),
            } satisfies Partial<BunRequest>;

            const registerController: RegisterController = createRegisterController(mockUserRepository, mockHash);

            // Act
            const actualResponse: Response = await registerController.post(expectedRequest as unknown as BunRequest);

            // Assert
            expect(expectedRequest.json).toHaveBeenCalled();
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
            expect(actualResponse.status).toEqual(expectedResponse.status);
        });

        test('returns 400 when no JSON provided', async () => {
            // Arrange
            const expectedResponseJson = {
                error: UNEXPECTED_END_OF_JSON_ERROR_MESSAGE,
            };
            const expectedResponseInit: ResponseInit = {
                headers: HEADERS,
                status: constants.HTTP_STATUS_BAD_REQUEST
            };
            const expectedResponse: Response = Response.json(expectedResponseJson, expectedResponseInit);

            const expectedRequest = {
                json: mock().mockRejectedValue(new Error(UNEXPECTED_END_OF_JSON_ERROR_MESSAGE)),
            } satisfies Partial<BunRequest>;

            const registerController: RegisterController = createRegisterController(mockUserRepository, mockHash);

            // Act
            const actualResponse: Response = await registerController.post(expectedRequest as unknown as BunRequest);

            // Assert
            expect(expectedRequest.json).toHaveBeenCalled();
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
            expect(actualResponse.status).toEqual(expectedResponse.status);
        });

        test('creates user with valid username and password', async () => {
            // Arrange
            const expectedUsername = 'username';
            const expectedPassword = 'password';
            const expectedPasswordHash = 'password_hash';
            const [expectedUserInsert, expectedAddedUser] = createUserFixture(expectedUsername, expectedPasswordHash);
            const expectedResponseJson: UserDto = {
                id: expectedAddedUser.id,
                username: expectedUsername,
                isActive: true,
            };
            const expectedResponse: Response = Response.json(expectedResponseJson, { headers: HEADERS });
            const expectedRequestJson = { username: expectedUsername, password: expectedPassword };

            const expectedRequest = {
                json: mock().mockResolvedValue(expectedRequestJson),
            } satisfies Partial<BunRequest>;

            mockHash.mockResolvedValue(expectedPasswordHash);
            mockAddUser.mockResolvedValue(expectedAddedUser);

            const registerController: RegisterController = createRegisterController(mockUserRepository, mockHash);

            // Act
            const actualResponse: Response = await registerController.post(expectedRequest as unknown as BunRequest);

            // Assert
            expect(expectedRequest.json).toHaveBeenCalled();
            expect(mockHash).toHaveBeenCalledWith(expectedPassword);
            expect(mockAddUser).toHaveBeenCalledWith(expectedUserInsert);
            expect(await actualResponse.json()).toEqual(expectedResponseJson);
            expect(actualResponse.headers.toJSON()).toEqual(expectedResponse.headers.toJSON());
            expect(actualResponse.status).toEqual(expectedResponse.status);
        });
    });
});
