import { createContainer, asFunction, InjectionMode, asValue } from 'awilix';
import { createRegisterController } from './controllers/auth/register';
import { createUserRepository } from './persistence/user.db';
import { sql } from './persistence/db';
import { createSessionRepository } from './persistence/session.kv';
import { kv } from './persistence/kv';
import { createLoginController } from './controllers/auth/login';
import { createGenerateRandomToken } from './utils/random';

const container = createContainer({
    injectionMode: InjectionMode.CLASSIC,
    strict: true,
});

container.register({
    registerController: asFunction(createRegisterController),
    loginController: asFunction(createLoginController),
    userRepository: asFunction(createUserRepository),
    sessionRepository: asFunction(createSessionRepository),
    generateRandomToken: asFunction(createGenerateRandomToken),
    cryptoProvider: asValue(crypto),
    hashFn: asValue(Bun.password.hash),
    verifyHash: asValue(Bun.password.verify),
    sql: asValue(sql),
    kv: asValue(kv),
});

export default container;
