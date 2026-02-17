export type UserSession = {
    userId: string,
};

export type UserSessionInsert = UserSession & {
    token: string,
}
