import { createContainer, asFunction, InjectionMode, asValue } from 'awilix';
import { createRegisterController } from './controllers/auth/register';
import { createUserRepository } from './persistence/user.db';
import { sql } from './persistence/db';
import { createSessionRepository } from './persistence/session.kv';
import { kv } from './persistence/kv';
import { createLoginController } from './controllers/auth/login';
import { createGenerateRandomToken } from './utils/random';
import { createProjectRepository } from './persistence/project.db';
import { createProjectResolver } from './graphql/resolvers/project';
import { createSchema, createYoga } from 'graphql-yoga';
import { getGraphQlServerInstance } from './graphql/server';
import { createGraphQlContext } from './graphql/context';
import { createAuthDecorators } from './decorators/auth';
import { withCors, withErrorHandling } from './decorators/controller';
import { preflight } from './controllers/preflight';

const container = createContainer({
    injectionMode: InjectionMode.PROXY,
    strict: true,
});

container.register({
    preflightController: asValue(preflight),
    registerController: asFunction(createRegisterController),
    loginController: asFunction(createLoginController),
    projectResolver: asFunction(createProjectResolver).singleton(),
    userRepository: asFunction(createUserRepository),
    sessionRepository: asFunction(createSessionRepository).singleton(),
    projectRepository: asFunction(createProjectRepository).singleton(),
    generateRandomToken: asFunction(createGenerateRandomToken),
    cryptoProvider: asValue(crypto),
    hashFn: asValue(Bun.password.hash),
    verifyHash: asValue(Bun.password.verify),
    sql: asValue(sql),
    kv: asValue(kv),
    createGraphQlServer: asValue(createYoga),
    createGraphQlSchema: asValue(createSchema),
    graphQlContextResolver: asFunction(createGraphQlContext).singleton(),
    graphQlServer: asFunction(getGraphQlServerInstance).singleton(),
    authDecorators: asFunction(createAuthDecorators).singleton(),
    withErrorHandling: asValue(withErrorHandling),
    withCors: asValue(withCors),
});

export default container;
