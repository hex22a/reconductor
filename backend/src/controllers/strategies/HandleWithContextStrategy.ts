import type { RequestContext, RequestHandler } from '@/src/controllers/types';
import type { IHandleStrategy } from './IHandleStrategy';
import type { BunRequest, MaybePromise } from 'bun';

export class HandleWithContextStrategy implements IHandleStrategy<RequestContext> {
    handle(
        handler: RequestHandler<RequestContext>,
        request: BunRequest,
        context: RequestContext,
    ): MaybePromise<Response> {
        return handler(request, context);
    }
}
