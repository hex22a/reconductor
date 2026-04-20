import type { BunRequest } from 'bun';
import type { ProjectResolver } from './project/project.resovler';
import type { GraphQLSchemaWithContext, YogaServerInstance } from 'graphql-yoga';
import type { GraphQlContext } from '../transport/graphql.context';
import type { ScanResolver } from './scan/scan.resolver';
import type { ScanRunResolver } from './scanRun/scanRun.resolver';
import type { HostResolver } from './host/host.resolver';

const typeDefs = await Bun.file('../shared/schema.graphql').text();

export type GraphQlUserContext = {
    userId: string | null;
};

export type GraphQlServerContext = {
    request: BunRequest;
};

export type GraphQlContextResolver = (
    serverContext: GraphQlServerContext,
) => Promise<GraphQlContext>;

export type GraphQlSchemaFactory = (options: {
    typeDefs: string;
    resolvers: unknown;
}) => GraphQLSchemaWithContext<never>;

export type GraphQlServerFactory<
    TServerCtx extends Record<string, unknown>,
    TUserCtx extends Record<string, unknown>,
> = (options: {
    schema: GraphQLSchemaWithContext<never>;
    context: GraphQlContextResolver;
}) => YogaServerInstance<TServerCtx, TUserCtx>;

export type GraphQlServerInstance = YogaServerInstance<GraphQlServerContext, GraphQlUserContext>;

export type GraphQlServerFactoryDeps = {
    createGraphQlServer: GraphQlServerFactory<GraphQlServerContext, GraphQlUserContext>;
    createGraphQlSchema: GraphQlSchemaFactory;
    graphQlContextResolver: GraphQlContextResolver;
    projectResolver: ProjectResolver;
    scanResolver: ScanResolver;
    scanRunResolver: ScanRunResolver;
    hostResolver: HostResolver;
};

export function getGraphQlServerInstance({
    createGraphQlServer,
    createGraphQlSchema,
    graphQlContextResolver,
    projectResolver,
    scanResolver,
    scanRunResolver,
    hostResolver,
}: GraphQlServerFactoryDeps) {
    return createGraphQlServer({
        schema: createGraphQlSchema({
            typeDefs,
            resolvers: [projectResolver, scanResolver, scanRunResolver, hostResolver],
        }),
        context: graphQlContextResolver,
    });
}

export type GraphQlServerFetch = ReturnType<typeof getGraphQlServerInstance>['fetch'];
