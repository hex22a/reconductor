import { describe, expect, test, it } from 'bun:test';
import { catchRollback, withTrx } from '../decorators';
import { createPortRepository, type PortRepository } from '@/src/persistence/port.db';
import { expectedExistingPortId, expectedPort, type PortEntity } from '@/src/domain/port.entity';
import { PortNotFoundError } from '@/src/domain/errors/PortNotFoundError';

describe('port.db', () => {
    test('createPortRepository', async () => {
        await catchRollback(async () => {
            await withTrx(async (trx) => {
                // Arrange
                const expectedPortRepository: PortRepository = {
                    getPort: expect.any(Function),
                };
                // Act
                const actualPortRepository: PortRepository = createPortRepository({ sql: trx });
                // Assert
                expect(actualPortRepository).toEqual(expectedPortRepository);
            });
        });
    });

    describe('getPort', () => {
        it('returns a port if exists', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const portRepository: PortRepository = createPortRepository({ sql: trx });
                    // Act
                    const actualPort: PortEntity =
                        await portRepository.getPort(expectedExistingPortId);
                    // Assert
                    expect(actualPort.id).toEqual(expectedExistingPortId);
                    expect(actualPort.port).toEqual(expectedPort);
                });
            });
        });

        it('throws an error if port not found', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedPortId = '019c8a70-1fb0-7d53-aef0-5e163f62fb24';
                    const portRepository: PortRepository = createPortRepository({ sql: trx });
                    // Act
                    try {
                        await portRepository.getPort(expectedPortId);
                    } catch (actualError) {
                        // Assert
                        expect(actualError).toBeInstanceOf(PortNotFoundError);
                    }
                });
            });
        });
    });
});
