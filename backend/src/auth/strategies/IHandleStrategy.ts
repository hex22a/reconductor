import type { RequestContext, RequestHandler } from '@/src/controllers/types';
import type { BunRequest, MaybePromise } from 'bun';

export interface IHandleStrategy<Context = void> {
    handle(
        handler: RequestHandler<Context>,
        request: BunRequest,
        context?: RequestContext,
    ): MaybePromise<Response>;
}
