import type { RequestHandler } from '@/src/controllers/types';
import type { IHandleStrategy } from './IHandleStrategy';

export class HandleCallStrategy implements IHandleStrategy {
    handle(handler: RequestHandler, request: Bun.BunRequest): Bun.MaybePromise<Response> {
        return handler(request);
    }
}
