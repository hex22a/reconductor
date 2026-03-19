export function encodeCursor(value: string) {
    return Buffer.from(value).toString('base64');
}

export function decodeCursor(cursor: string) {
    return Buffer.from(cursor, 'base64').toString();
}
