import { TOKEN_RANDOM_BYTES_ARRAY_LENGTH } from "../constants"

export function createGenerateRandomToken(randomBytes: Function): () => string {
    return function generateRandomToken(): string {
        const bytes: Uint8Array = randomBytes(new Uint8Array([TOKEN_RANDOM_BYTES_ARRAY_LENGTH]));
        return Buffer.from(bytes).toString('base64url');
    }
};
