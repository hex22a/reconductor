import { afterEach, describe, expect, mock, test } from 'bun:test';
import {
    createQueueService,
    type QueueService,
    type QueueServiceFactoryDeps,
} from './queue.service';
import type { MqProvider } from './mq';
import { RABBITMQ_URL, SCAN_QUEUE } from '../constants';
import type { Channel } from 'amqplib';

describe('queue.service', () => {
    const mockConnect = mock();
    const mockCreateChannel = mock();
    const mockSendToQueue = mock();
    const mockAssertQueue = mock();
    const mockCloseConnection = mock();
    const mockCloseChannel = mock();
    const mockConnection = {
        createChannel: mockCreateChannel,
        close: mockCloseConnection,
    };
    const mockMqProvider: Partial<MqProvider> = {
        connect: mockConnect,
    };
    const mockChannel: Partial<Channel> = {
        assertQueue: mockAssertQueue,
        sendToQueue: mockSendToQueue,
        close: mockCloseChannel,
    };
    const expectedQueueServiceDeps: QueueServiceFactoryDeps = {
        mq: mockMqProvider as unknown as MqProvider,
    };

    afterEach(() => {
        mockConnect.mockReset();
        mockCreateChannel.mockReset();
        mockSendToQueue.mockReset();
        mockAssertQueue.mockReset();
        mockCloseChannel.mockReset();
        mockCloseConnection.mockReset();
    });

    test('creatreQueueService', async () => {
        // Arrange
        const expectedQueueService: QueueService = {
            publish: expect.any(Function),
            close: expect.any(Function),
        };
        mockConnect.mockResolvedValue(mockConnection);
        // Act
        const actualQueueService: QueueService = await createQueueService(expectedQueueServiceDeps);
        // Assert
        expect(actualQueueService).toEqual(expectedQueueService);
        expect(mockConnect).toHaveBeenLastCalledWith(RABBITMQ_URL);
        expect(mockCreateChannel).toHaveBeenCalled();
    });

    test('publish', async () => {
        // Arrange
        const expectedMessage = 'start scan';
        const expectedMessageBuffer = Buffer.from(expectedMessage);
        mockConnect.mockResolvedValue(mockConnection);
        mockCreateChannel.mockResolvedValue(mockChannel);
        const queueService: QueueService = await createQueueService(expectedQueueServiceDeps);
        // Act
        await queueService.publish(expectedMessage);
        // Assert
        expect(mockAssertQueue).toHaveBeenCalledWith(SCAN_QUEUE, { durable: true });
        expect(mockSendToQueue).toHaveBeenCalledWith(SCAN_QUEUE, expectedMessageBuffer);
    });

    test('close', async () => {
        // Arrange
        mockConnect.mockResolvedValue(mockConnection);
        mockCreateChannel.mockResolvedValue(mockChannel);
        const queueService: QueueService = await createQueueService(expectedQueueServiceDeps);
        // Act
        await queueService.close();
        // Assert
        expect(mockCloseChannel).toHaveBeenCalled();
        expect(mockCloseConnection).toHaveBeenCalled();
    });
});
