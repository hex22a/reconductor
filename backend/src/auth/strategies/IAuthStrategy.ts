import type { UserSession } from '@/src/domain/session.entity';
import type { BunRequest } from 'bun';

export interface IAuthStrategy {
    authenticate(request: BunRequest): Promise<UserSession>;
}
