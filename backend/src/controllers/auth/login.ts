import { HEADERS, USER_SESSION_COOKIE_MAX_AGE, USER_SESSION_COOKIE_NAME } from "@/src/constants";
import type { UserEntity } from "@/src/domain/user.entity";
import type { SessionRepository } from "@/src/persistence/session.kv";
import type { UserRepository } from "@/src/persistence/user.db";
import { registerSchema } from "@/src/transport/user.schema";
import type { BunRequest } from "bun";

export type LoginController = {
    post: (req: BunRequest) => Promise<Response>;
};

export function createLoginController(
    userRepository: UserRepository,
    sessionRepository: SessionRepository,
    verifyHash: (password: string, hash: string) => Promise<boolean>,
    getRandomToken: () => string,
): LoginController {
    return {
        async post(req: BunRequest): Promise<Response> {
            const requestJson = await req.json();
            const { username, password } = registerSchema.parse(requestJson);
            const user: UserEntity = await userRepository.getUserByUsername(username);
            verifyHash(password, user.password_hash);
            const token = getRandomToken();
            sessionRepository.createUserSession({ token, userId: user.id });
            req.cookies.set(USER_SESSION_COOKIE_NAME, token, {
                maxAge: USER_SESSION_COOKIE_MAX_AGE,
                httpOnly: true,
                secure: true,
                path: '/',
            });
            return Response.json({ ok: true }, { headers: HEADERS });
        },
    };
}
