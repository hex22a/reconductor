import {
    CSRF_SECRET,
    TOKEN_RANDOM_BYTES_ARRAY_LENGTH,
    USER_SESSION_TTL_MILLISECONDS,
} from '../constants';
import type { CsrfProvider } from './csrf';

export type TokenPorviderFactoryDeps = {
    cryptoProvider: Crypto;
    csrfProvider: CsrfProvider;
};

export type TokenProvider = {
    generateRandomToken(): string;
    generateCsrfToken(): string;
    verifyCsrfToken(token: string): boolean;
};

export function createTokenProvider({
    cryptoProvider,
    csrfProvider,
}: TokenPorviderFactoryDeps): TokenProvider {
    return {
        generateRandomToken(): string {
            const bytes: Uint8Array = cryptoProvider.getRandomValues(
                new Uint8Array(TOKEN_RANDOM_BYTES_ARRAY_LENGTH),
            );
            return Buffer.from(bytes).toString('base64url');
        },
        generateCsrfToken(): string {
            return csrfProvider.generate(CSRF_SECRET, {
                expiresIn: USER_SESSION_TTL_MILLISECONDS,
                encoding: 'base64url',
                algorithm: 'sha256',
            });
        },
        verifyCsrfToken(token: string): boolean {
            return csrfProvider.verify(token, {
                secret: CSRF_SECRET,
                maxAge: USER_SESSION_TTL_MILLISECONDS,
                encoding: 'base64url',
                algorithm: 'sha256',
            });
        },
    };
}
