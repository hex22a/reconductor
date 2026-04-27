import type { CsrfProvider } from '@/src/providers/csrf';
import type { BunRequest, MaybePromise } from 'bun';
import type { RequestContext } from '../types';

export interface ICsrfStrategy {
    csrfProvider: CsrfProvider;
    verifyCsrfToken(request: BunRequest, context?: RequestContext): MaybePromise<boolean>;
}
