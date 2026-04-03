import type { Channel, ChannelModel } from 'amqplib';
import { RABBITMQ_URL, SCAN_QUEUE } from '../constants';
import type { ScanMessageDto } from '../transport/scan.dto';
import type { MqProvider } from './mq';

export type QueueServiceFactoryDeps = {
    mq: MqProvider;
};

export type QueueService = {
    ensureConnected: () => Promise<void>;
    publish: (message: ScanMessageDto) => Promise<void>;
    close: () => Promise<void>;
};

export function createQueueService({ mq }: QueueServiceFactoryDeps): QueueService {
    let connection: ChannelModel;
    let channel: Channel;
    async function connect() {
        connection = await mq.connect(RABBITMQ_URL);
        channel = await connection.createChannel();
    }
    return {
        async ensureConnected() {
            if (!channel) {
                await connect();
            }
        },
        async publish(message: ScanMessageDto) {
            await this.ensureConnected();
            await channel.assertQueue(SCAN_QUEUE, { durable: true });
            channel.sendToQueue(SCAN_QUEUE, Buffer.from(JSON.stringify(message)));
        },
        async close() {
            await channel?.close();
            await connection?.close();
        },
    };
}
