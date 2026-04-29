import type { BunRequest, MaybePromise } from 'bun';
import type { RequestContext } from '../types';
import type { TokenProvider } from '@/src/providers/token';

export interface ICsrfStrategy<Context extends RequestContext | void> {
    tokenProvider: TokenProvider;
    verifyCsrfToken(request: BunRequest, context: Context): MaybePromise<boolean>;
}
