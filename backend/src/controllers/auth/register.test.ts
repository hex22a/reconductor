import type { BunRequest } from 'bun';
import { describe, test, expect, mock, afterEach } from 'bun:test';
import { HEADERS } from '../../constants';
import { createRegisterController, type RegisterController, type RegisterControllerDeps } from './register';
import type { UserDto } from '@/src/transport/user.dto';
import { createUserFixture } from '@/tests/fixtures/users';
import type { UserRepository } from '@/src/persistence/user.db';

describe('register', () => {
    const mockHash = mock();
    const mockAddUser = mock();
    const mockGetUserByUsername = mock();
    const mockUserRepository: UserRepository = {
        addUser: mockAddUser,
        getUserByUsername: mockGetUserByUsername,
    };
    const expectedRegisterControllerDeps: RegisterControllerDeps = {
        userRepository: mockUserRepository,
        hashFn: mockHash,
    };

    afterEach(() => {
        mockHash.mockReset();
        mockAddUser.mockReset();
        mockGetUserByUsername.mockReset();
    });

    test('createRegisterController', () => {
        // Arrange
        // Act
        const actualRegisterController: RegisterController = createRegisterController(expectedRegisterControllerDeps);
        // Assert
        expect(actualRegisterController.post).toBeFunction();

    });

    test('register controller', async () => {
        // Arrange
        const expectedUsername = 'username';
        const expectedPassword = 'password';
        const expectedPasswordHash = 'password_hash';
        const [expectedAddedUser, expectedUserInsert] = createUserFixture(expectedUsername, expectedPasswordHash);
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

        const registerController: RegisterController = createRegisterController(expectedRegisterControllerDeps);

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
