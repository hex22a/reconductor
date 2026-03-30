import type { ValidationError } from './error.dto';

export type CreateEntityPayload<E> = {
    edge?: E;
    errors: ValidationError[];
};
