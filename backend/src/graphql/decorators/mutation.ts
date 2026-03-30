import type { GraphQlContext } from '@/src/transport/graphql.context';
import type { MutationResolver } from '../types';
import { ZodError } from 'zod';
import type { CreateEntityPayload } from '@/src/transport/payload.dto';
import type { Edge } from '@/src/transport/edge.dto';
import type { ValidationError } from '@/src/transport/error.dto';

export function withValidation<T, A>(mutation: MutationResolver<T, A>): MutationResolver<T, A> {
    return async function (
        parent: unknown,
        args: A,
        context: GraphQlContext,
    ): Promise<CreateEntityPayload<Edge<T>>> {
        try {
            return await mutation(parent, args, context);
        } catch (error) {
            if (error instanceof ZodError) {
                const errors: Array<ValidationError> = error.issues.map((issue) => ({
                    field: String(issue.path[0] ?? 'unknown'),
                    message: issue.message,
                }));
                return {
                    errors,
                };
            }
            return {
                errors: [],
            };
        }
    };
}
