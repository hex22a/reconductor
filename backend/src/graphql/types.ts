import type { Edge } from '@/src/transport/edge.dto';
import type { GraphQlContext } from '@/src/transport/graphql.context';
import type { Pagination } from '@/src/transport/pagination.dto';
import type { CreateEntityPayload } from '@/src/transport/payload.dto';

export type EntityResolver<T, A> = (parent: unknown, args: A) => Promise<T>;

export type PaginatonResolver<T, A = unknown> = (
    parent: unknown,
    args: A,
    context: GraphQlContext,
) => Promise<Pagination<Edge<T>>>;

export type MutationResolver<T, A> = (
    parent: unknown,
    args: A,
    context: GraphQlContext,
) => Promise<CreateEntityPayload<Edge<T>>>;
