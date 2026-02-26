import { TOKEN_RANDOM_BYTES_ARRAY_LENGTH } from '../constants';

export type GenerateRandomTokenFactoryDeps = {
    cryptoProvider: Crypto;
};

export function createGenerateRandomToken({
    cryptoProvider,
}: GenerateRandomTokenFactoryDeps): () => string {
    return function generateRandomToken(): string {
        const bytes: Uint8Array = cryptoProvider.getRandomValues(
            new Uint8Array(TOKEN_RANDOM_BYTES_ARRAY_LENGTH),
        );
        return Buffer.from(bytes).toString('base64url');
    };
}
