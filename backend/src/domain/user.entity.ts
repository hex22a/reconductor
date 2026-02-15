export type UserEntity = {
    id: string,
    username: string,
    password_hash: string,
    password_version: number,
    created_at: Date,
    updated_at: Date,
    last_login_at: Date,
    is_active: boolean,
};

export type UserInsert = Pick<UserEntity, 'username' | 'password_hash'>;
