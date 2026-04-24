import { USER_SESSION_TTL_SECONDS, USER_SESSION_COOKIE_NAME } from '@/src/constants';
import type { UserEntity } from '@/src/domain/user.entity';
import type { SessionRepository } from '@/src/persistence/session.kv';
import type { UserRepository } from '@/src/persistence/user.db';
import { registerSchema } from '@/src/transport/user.schema';
import type { BunRequest } from 'bun';
import { constants } from 'node:http2';
import type { RequestHandler } from '../types';
import type { TokenProvider } from '@/src/providers/token';

export type LoginControllerDeps = {
    userRepository: UserRepository;
    sessionRepository: SessionRepository;
    verifyHash: (password: string, hash: string) => Promise<boolean>;
    tokenProvider: TokenProvider;
};

export type LoginController = {
    post: RequestHandler<void>;
};

export function createLoginController({
    userRepository,
    sessionRepository,
    verifyHash,
    tokenProvider,
}: LoginControllerDeps): LoginController {
    return {
        async post(req: BunRequest): Promise<Response> {
            const requestJson = await req.json();
            const { username, password } = registerSchema.parse(requestJson);
            const user: UserEntity = await userRepository.getUserByUsername(username);
            const passwordsMatch = await verifyHash(password, user.password_hash);
            if (!passwordsMatch) {
                return Response.json({ ok: false }, { status: constants.HTTP_STATUS_UNAUTHORIZED });
            }
            const token = tokenProvider.generateRandomToken();
            const csrfToken = tokenProvider.generateCsrfToken();
            await sessionRepository.createUserSession({
                token,
                csrfToken,
                userId: user.id,
                username: user.username,
            });
            req.cookies.set(USER_SESSION_COOKIE_NAME, token, {
                maxAge: USER_SESSION_TTL_SECONDS,
                httpOnly: true,
                secure: true,
                path: '/',
            });
            return Response.json({ ok: true, csrfToken });
        },
    };
}
