import { CSRF_HEADER } from '@/src/constants';
import type { ICsrfStrategy } from './ICsrfStrategy';
import type { CsrfRepository } from '@/src/persistence/csrf.kv';
import type { BunRequest } from 'bun';
import type { TokenProvider } from '@/src/providers/token';

export type AnonymousCsrfStrategyDeps = {
    tokenProvider: TokenProvider;
    csrfRepository: CsrfRepository;
};

export class AnonymousCsrfStrategy implements ICsrfStrategy<void> {
    tokenProvider: TokenProvider;
    csrfRepository: CsrfRepository;

    constructor({ csrfRepository, tokenProvider }: AnonymousCsrfStrategyDeps) {
        this.tokenProvider = tokenProvider;
        this.csrfRepository = csrfRepository;
    }

    async verifyCsrfToken(req: BunRequest): Promise<boolean> {
        const csrfToken = req.headers.get(CSRF_HEADER);
        return (
            !!csrfToken &&
            this.tokenProvider.verifyCsrfToken(csrfToken) &&
            (await this.csrfRepository.verifyAnonymousCsrf(csrfToken))
        );
    }
}
