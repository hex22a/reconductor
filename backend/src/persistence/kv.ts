import { redis, type RedisClient } from 'bun';

export type KvClinent = RedisClient;
export const kv = redis;
