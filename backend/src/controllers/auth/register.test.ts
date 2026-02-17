import type { BunRequest } from 'bun';
import { constants } from 'node:http2';
import { describe, test, expect, mock } from 'bun:test';
import { HEADERS, UNEXPECTED_END_OF_JSON_ERROR_MESSAGE, UNEXPECTED_ERROR_MESSAGE, Z_REGISTER_SCHEMA_ERROR_MESSAGE } from '../../constants';
import { createRegisterController, type RegisterController } from './register';
import type { UserDto } from '@/src/transport/user.dto';
import { createUserFixture } from '@/tests/fixtures/users';
import { afterEach } from 'node:test';
import type { ErrorResponse } from '@/src/transport/error.dto';

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

    test('register controller', async () => {
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
