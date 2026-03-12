import type { RequestHandler } from '@/src/controllers/types';
import type { BunRequest } from 'bun';
import type { GraphQlServerFetch } from '../server';

export function toHandler(fetch: GraphQlServerFetch): RequestHandler<void> {
    return function handle(request: BunRequest) {
        return fetch(request);
    };
}

export type FetchToHandlerAdapter = typeof toHandler;
