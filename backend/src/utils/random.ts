import { TOKEN_RANDOM_BYTES_ARRAY_LENGTH } from "../constants";

export function createGenerateRandomToken(cryptoProvider: Crypto): () => string {
    return function generateRandomToken(): string {
        const bytes: Uint8Array = cryptoProvider.getRandomValues(new Uint8Array(TOKEN_RANDOM_BYTES_ARRAY_LENGTH));
        return Buffer.from(bytes).toString('base64url');
    };
};
