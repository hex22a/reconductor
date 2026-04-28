import type { BunRequest, MaybePromise } from 'bun';
import type { UserSession } from '../domain/session.entity';

export type RequestContext = {
    user: UserSession;
};

export type RequestHandler<Context extends RequestContext | void> = Context extends void
    ? (req: BunRequest) => MaybePromise<Response>
    : (req: BunRequest, context: Context) => MaybePromise<Response>;
