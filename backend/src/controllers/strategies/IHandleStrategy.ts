import type { RequestContext, RequestHandler } from '@/src/controllers/types';
import type { BunRequest, MaybePromise } from 'bun';

export interface IHandleStrategy<Context extends RequestContext | void> {
    handle(
        handler: RequestHandler<Context>,
        request: BunRequest,
        context: Context,
    ): MaybePromise<Response>;
}
