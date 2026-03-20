export function encodeCursor(value: string) {
    return Buffer.from(value).toString('base64');
}

export function decodeCursor(cursor: string) {
    return Buffer.from(cursor, 'base64').toString();
}

export type CursorEncoder = typeof encodeCursor;
export type CursorDecoder = typeof decodeCursor;
