import type { RequestHandler } from '@/src/controllers/types';
import type { IHandleStrategy } from './IHandleStrategy';
import type { BunRequest, MaybePromise } from 'bun';

export class HandleCallStrategy implements IHandleStrategy {
    handle(handler: RequestHandler, request: BunRequest): MaybePromise<Response> {
        return handler(request);
    }
}
