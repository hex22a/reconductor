import type { RequestHandler } from '@/src/controllers/types';
import type { IHandleStrategy } from './IHandleStrategy';
import type { BunRequest, MaybePromise } from 'bun';

export class HandleCallStrategy implements IHandleStrategy<void> {
    handle(handler: RequestHandler<void>, request: BunRequest): MaybePromise<Response> {
        return handler(request);
    }
}
