export type UserSession = {
    userId: string;
    username: string;
};

export type UserSessionInsert = UserSession & {
    token: string;
};
