import { RABBITMQ_URL, SCAN_QUEUE } from '../constants';
import type { MqProvider } from './mq';

export type QueueServiceFactoryDeps = {
    mq: MqProvider;
};

export type QueueService = {
    publish: (message: string) => Promise<void>;
    close: () => Promise<void>;
};

export async function createQueueService({ mq }: QueueServiceFactoryDeps): Promise<QueueService> {
    const connection = await mq.connect(RABBITMQ_URL);
    const channel = await connection.createChannel();
    return {
        async publish(message: string) {
            await channel.assertQueue(SCAN_QUEUE, { durable: true });
            channel.sendToQueue(SCAN_QUEUE, Buffer.from(message));
        },
        async close() {
            await channel.close();
            await connection.close();
        },
    };
}
