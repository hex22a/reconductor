import { afterEach, describe, expect, mock, test } from 'bun:test';
import {
    createQueueService,
    type QueueService,
    type QueueServiceFactoryDeps,
} from './queue.service';
import type { MqProvider } from './mq';
import { RABBITMQ_URL, SCAN_QUEUE } from '../constants';
import type { Channel } from 'amqplib';
import type { ScanMessageDto } from '../transport/scan.dto';

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

    test('creatreQueueService', () => {
        // Arrange
        const expectedQueueService: QueueService = {
            ensureConnected: expect.any(Function),
            publish: expect.any(Function),
            close: expect.any(Function),
        };
        // Act
        const actualQueueService: QueueService = createQueueService(expectedQueueServiceDeps);
        // Assert
        expect(actualQueueService).toEqual(expectedQueueService);
    });

    test('ensureConnected', async () => {
        // Arrange
        mockConnect.mockResolvedValue(mockConnection);
        mockCreateChannel.mockResolvedValue(mockChannel);
        const queueService: QueueService = createQueueService(expectedQueueServiceDeps);
        // Act
        await queueService.ensureConnected();
        // Assert
        expect(mockConnect).toHaveBeenCalledWith(RABBITMQ_URL);
        expect(mockCreateChannel).toHaveBeenCalled();
    });

    test('publish', async () => {
        // Arrange
        const expectedId = '019d4f1c-afb4-7fd5-8afe-361b489732f5';
        const expectedTarget = '192.168.1.0/16';
        const expectedMessage: ScanMessageDto = {
            id: expectedId,
            target: expectedTarget,
        };
        const expectedMessageBuffer = Buffer.from(JSON.stringify(expectedMessage));
        mockConnect.mockResolvedValue(mockConnection);
        mockCreateChannel.mockResolvedValue(mockChannel);
        const queueService: QueueService = createQueueService(expectedQueueServiceDeps);
        // Act
        await queueService.publish(expectedMessage);
        // Assert
        expect(mockConnect).toHaveBeenCalledWith(RABBITMQ_URL);
        expect(mockCreateChannel).toHaveBeenCalled();
        expect(mockAssertQueue).toHaveBeenCalledWith(SCAN_QUEUE, { durable: true });
        expect(mockSendToQueue).toHaveBeenCalledWith(SCAN_QUEUE, expectedMessageBuffer);
    });

    test('close', async () => {
        // Arrange
        mockConnect.mockResolvedValue(mockConnection);
        mockCreateChannel.mockResolvedValue(mockChannel);
        const queueService: QueueService = createQueueService(expectedQueueServiceDeps);
        await queueService.ensureConnected();
        // Act
        await queueService.close();
        // Assert
        expect(mockCloseChannel).toHaveBeenCalled();
        expect(mockCloseConnection).toHaveBeenCalled();
    });
});
