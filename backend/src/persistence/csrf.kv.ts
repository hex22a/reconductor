import { ANONYMOUS_CSRF_PREFIX, ANONYMOUS_CSRF_TTL_SECONDS } from '../constants';
import type { KvClinent } from './kv';

export type CsrfRepositoryDeps = {
    kv: KvClinent;
};

export type CsrfRepository = {
    createAnonymousCsrf: (token: string) => Promise<boolean>;
    checkAnonymousCsrf: (token: string) => Promise<boolean>;
    deleteAnonymousCsrf: (token: string) => Promise<void>;
};

export function createCsrfRepository({ kv }: CsrfRepositoryDeps): CsrfRepository {
    return {
        async createAnonymousCsrf(token: string): Promise<boolean> {
            const key = `${ANONYMOUS_CSRF_PREFIX}:${token}`;
            await kv.setbit(key, 0, 1);
            await kv.expire(key, ANONYMOUS_CSRF_TTL_SECONDS);
            return true;
        },
        async checkAnonymousCsrf(token: string): Promise<boolean> {
            const key = `${ANONYMOUS_CSRF_PREFIX}:${token}`;
            const bit = await kv.getbit(key, 0);
            return bit === 1;
        },
        async deleteAnonymousCsrf(token: string): Promise<void> {
            const key = `${ANONYMOUS_CSRF_PREFIX}:${token}`;
            await kv.del(key);
        },
    };
}
