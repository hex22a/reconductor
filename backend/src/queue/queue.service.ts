import { RABBITMQ_URL, SCAN_QUEUE } from '../constants';
import type { ScanMessageDto } from '../transport/scan.dto';
import type { MqProvider } from './mq';

export type QueueServiceFactoryDeps = {
    mq: MqProvider;
};

export type QueueService = {
    publish: (message: ScanMessageDto) => Promise<void>;
    close: () => Promise<void>;
};

export async function createQueueService({ mq }: QueueServiceFactoryDeps): Promise<QueueService> {
    const connection = await mq.connect(RABBITMQ_URL);
    const channel = await connection.createChannel();
    return {
        async publish(message: ScanMessageDto) {
            await channel.assertQueue(SCAN_QUEUE, { durable: true });
            channel.sendToQueue(SCAN_QUEUE, Buffer.from(JSON.stringify(message)));
        },
        async close() {
            await channel.close();
            await connection.close();
        },
    };
}
