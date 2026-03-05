import type { BunRequest } from 'bun';
import type { RequestContext } from '../types';

export function me(_: BunRequest, context: RequestContext): Response {
    return Response.json({ username: context.user.username });
}
