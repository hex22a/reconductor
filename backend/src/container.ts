import { createContainer, asFunction, InjectionMode, asValue } from 'awilix';
import { createRegisterController } from './controllers/auth/register';
import { createUserRepository } from './persistence/user.db';
import { sql } from './persistence/db';

const container = createContainer({
    injectionMode: InjectionMode.CLASSIC,
    strict: true,
});

container.register({
    registerController: asFunction(createRegisterController),
    userRepository: asFunction(createUserRepository),
    hashFn: asValue(Bun.password.hash),
    sql: asValue(sql),
});

export default container;
