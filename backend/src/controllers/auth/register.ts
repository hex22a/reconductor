import type { BunRequest } from "bun";
import { HEADERS, UNEXPECTED_ERROR_MESSAGE } from "../../constants";
import { constants } from 'node:http2';
import type { UserRepository } from "@/src/persistence/user.db";
import type { RegisterUserDto, UserDto } from "@/src/transport/user.dto";
import { registerSchema } from "@/src/transport/user.schema";
import type { UserEntity } from "@/src/domain/user.entity";

export type RegisterController = {
    post: (req: BunRequest) => Promise<Response>;
};

export function createRegisterController(
    userRepository: UserRepository,
    hash: (str: string) => Promise<string>,
): RegisterController {
    return {
        async post(req: BunRequest): Promise<Response> {
            try {
                const reqJson = await req.json();
                const { username, password }: RegisterUserDto = registerSchema.parse(reqJson);
                const password_hash = await hash(password);
                const userEntity: UserEntity = await userRepository.addUser({ username, password_hash });
                const userResponse: UserDto = {
                    id: userEntity.id,
                    username,
                    isActive: userEntity.is_active,
                };
                return Response.json(userResponse, { headers: HEADERS });
            } catch (error) {
                if (error instanceof Error) {
                    return Response.json(
                        { error: error.message },
                        { headers: HEADERS, status: constants.HTTP_STATUS_BAD_REQUEST }
                    )
                }
                return Response.json(
                    { error: UNEXPECTED_ERROR_MESSAGE },
                    { headers: HEADERS, status: constants.HTTP_STATUS_INTERNAL_SERVER_ERROR }
                )
            }
        }
    }
}

