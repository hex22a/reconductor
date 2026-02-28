import type { BunRequest } from 'bun';
import type { UserRepository } from '@/src/persistence/user.db';
import type { RegisterUserDto, UserDto } from '@/src/transport/user.dto';
import { registerSchema } from '@/src/transport/user.schema';
import type { UserEntity } from '@/src/domain/user.entity';

export type RegisterControllerDeps = {
    userRepository: UserRepository;
    hashFn: (str: string) => Promise<string>;
};

export type RegisterController = {
    post: (req: BunRequest) => Promise<Response>;
};

export function createRegisterController({
    userRepository,
    hashFn,
}: RegisterControllerDeps): RegisterController {
    return {
        async post(req: BunRequest): Promise<Response> {
            const reqJson = await req.json();
            const { username, password }: RegisterUserDto = registerSchema.parse(reqJson);
            const password_hash = await hashFn(password);
            const userEntity: UserEntity = await userRepository.addUser({
                username,
                password_hash,
            });
            const userResponse: UserDto = {
                id: userEntity.id,
                username,
                isActive: userEntity.is_active,
            };
            return Response.json(userResponse);
        },
    };
}
