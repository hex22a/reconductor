import { CSRF_HEADER } from '@/src/constants';
import type { ICsrfStrategy } from './ICsrfStrategy';
import type { CsrfRepository } from '@/src/persistence/csrf.kv';
import type { CsrfProvider } from '@/src/providers/csrf';
import type { BunRequest } from 'bun';

export type AnonymousCsrfStrategyDeps = {
    csrfProvider: CsrfProvider;
    csrfRepository: CsrfRepository;
};

export class AnonymousCsrfStrategy implements ICsrfStrategy {
    csrfProvider: CsrfProvider;
    csrfRepository: CsrfRepository;

    constructor({ csrfRepository, csrfProvider }: AnonymousCsrfStrategyDeps) {
        this.csrfProvider = csrfProvider;
        this.csrfRepository = csrfRepository;
    }

    async verifyCsrfToken(req: BunRequest): Promise<boolean> {
        const csrfToken = req.headers.get(CSRF_HEADER);
        return (
            !!csrfToken &&
            this.csrfProvider.verify(csrfToken) &&
            (await this.csrfRepository.verifyAnonymousCsrf(csrfToken))
        );
    }
}
