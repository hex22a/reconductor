import type { ProjectResolver } from '../resolvers/project';
import type { GraphQLSchemaWithContext, YogaServerInstance } from 'graphql-yoga';

const typeDefs = await Bun.file('../shared/schema.graphql').text();

export type GraphQlUserContext = {
    userId: string | null;
};

export type GraphQlServerContext = {
    request: Request;
};

export type GraphQlSchemaFactory = (options: {
    typeDefs: string;
    resolvers: unknown;
}) => GraphQLSchemaWithContext<never>;

export type GraphQlServerFactory<
    TServerCtx extends Record<string, unknown>,
    TUserCtx extends Record<string, unknown>,
> = (options: {
    schema: GraphQLSchemaWithContext<never>;
}) => YogaServerInstance<TServerCtx, TUserCtx>;

export type GraphQlServerInstance = YogaServerInstance<GraphQlServerContext, GraphQlUserContext>;

export type GraphQlServerFactoryDeps = {
    createGraphQlServer: GraphQlServerFactory<GraphQlServerContext, GraphQlUserContext>;
    createGraphQlSchema: GraphQlSchemaFactory;
    projectResolver: ProjectResolver;
};

export function getGraphQlServerInstance({
    createGraphQlServer,
    createGraphQlSchema,
    projectResolver,
}: GraphQlServerFactoryDeps) {
    return createGraphQlServer({
        schema: createGraphQlSchema({
            typeDefs,
            resolvers: projectResolver,
        }),
    });
}
