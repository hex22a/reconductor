import { GraphQLError } from 'graphql/error';
import {
    GRAPHQL_ERROR_EXTENSION_CODE,
    GRAPHQL_UNAUTHORIZED_ERROR_MESSAGE,
    USER_SESSION_COOKIE_NAME,
} from '../constants';
import type { UserSession } from '../domain/session.entity';
import type { SessionRepository } from '../persistence/session.kv';
import type { GraphQlContext } from '../transport/graphql.context';
import type { GraphQlContextResolver, GraphQlServerContext } from './server';

export type GraphQlContextFactoryDeps = {
    sessionRepository: SessionRepository;
};

export function createGraphQlContext({
    sessionRepository,
}: GraphQlContextFactoryDeps): GraphQlContextResolver {
    return async function context({ request }: GraphQlServerContext): Promise<GraphQlContext> {
        const token = request.cookies.get(USER_SESSION_COOKIE_NAME);
        if (!token) {
            throw new GraphQLError(GRAPHQL_UNAUTHORIZED_ERROR_MESSAGE, {
                extensions: { code: GRAPHQL_ERROR_EXTENSION_CODE },
            });
        }
        try {
            const { userId }: UserSession = await sessionRepository.getUserSession(token);
            return { user: { id: userId } };
        } catch {
            throw new GraphQLError(GRAPHQL_UNAUTHORIZED_ERROR_MESSAGE, {
                extensions: { code: GRAPHQL_ERROR_EXTENSION_CODE },
            });
        }
    };
}
