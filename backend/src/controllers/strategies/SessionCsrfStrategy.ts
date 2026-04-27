import type { BunRequest, MaybePromise } from 'bun';
import type { ICsrfStrategy } from './ICsrfStrategy';
import type { CsrfProvider } from '@/src/providers/csrf';
import type { RequestContext } from '../types';
import { CSRF_HEADER } from '@/src/constants';

export type SessionCsrfStrategyDeps = {
    csrfProvider: CsrfProvider;
};

export class SessionCsrfStrategy implements ICsrfStrategy {
    csrfProvider: CsrfProvider;

    constructor({ csrfProvider }: SessionCsrfStrategyDeps) {
        this.csrfProvider = csrfProvider;
    }

    verifyCsrfToken(req: BunRequest, context: RequestContext): MaybePromise<boolean> {
        const csrfToken = req.headers.get(CSRF_HEADER);
        return (
            !!csrfToken &&
            this.csrfProvider.verify(csrfToken) &&
            csrfToken === context.user.csrfToken
        );
    }
}
