import type { BunRequest, MaybePromise } from 'bun';
import type { ICsrfStrategy } from './ICsrfStrategy';
import type { RequestContext } from '../types';
import { CSRF_HEADER } from '@/src/constants';
import type { TokenProvider } from '@/src/providers/token';

export type SessionCsrfStrategyDeps = {
    tokenProvider: TokenProvider;
};

export class SessionCsrfStrategy implements ICsrfStrategy<RequestContext> {
    tokenProvider: TokenProvider;

    constructor({ tokenProvider }: SessionCsrfStrategyDeps) {
        this.tokenProvider = tokenProvider;
    }

    verifyCsrfToken(req: BunRequest, context: RequestContext): MaybePromise<boolean> {
        const csrfToken = req.headers.get(CSRF_HEADER);
        return (
            !!csrfToken &&
            this.tokenProvider.verifyCsrfToken(csrfToken) &&
            csrfToken === context.user.csrfToken
        );
    }
}
