import type { GraphQlContext } from '@/src/transport/graphql.context';
import type { MutationResolver } from '../resolvers/types';

export function withValidation<T, A>(mutation: MutationResolver<T, A>): MutationResolver<T, A> {
    return function (parent: unknown, args: A, context: GraphQlContext) {
        return mutation(parent, args, context);
    };
}
