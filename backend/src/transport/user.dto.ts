import { z } from 'zod';
import type { registerSchema } from './user.schema';

export type RegisterUserDto = z.infer<typeof registerSchema>;

export type UserDto = {
    id: string;
    username: string;
    isActive: boolean;
};
