import { describe, expect, test, it } from 'bun:test';
import { catchRollback, withTrx } from '../decorators';
import { createHostRepository, type HostRepository } from '@/src/persistence/host.db';
import { expectedExistingHostId, expectedHostIp, type HostEntity } from '@/src/domain/host.entity';
import { HostNotFoundError } from '@/src/domain/errors/HostNotFoundError';

describe('host.db', () => {
    test('createHostRepository', async () => {
        await catchRollback(async () => {
            await withTrx(async (trx) => {
                // Arrange
                const expectedHostRepository: HostRepository = {
                    getHost: expect.any(Function),
                };
                // Act
                const actualHostRepository: HostRepository = createHostRepository({ sql: trx });
                // Assert
                expect(actualHostRepository).toEqual(expectedHostRepository);
            });
        });
    });

    describe('getHost', () => {
        it('returns a host if exists', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const hostRepository: HostRepository = createHostRepository({ sql: trx });
                    // Act
                    const actualHost: HostEntity =
                        await hostRepository.getHost(expectedExistingHostId);
                    // Assert
                    expect(actualHost.id).toEqual(expectedExistingHostId);
                    expect(actualHost.ip).toEqual(expectedHostIp);
                });
            });
        });

        it('throws an error if host not found', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedHostId = '019c8a70-1fb0-7d53-aef0-5e163f62fb24';
                    const hostRepository: HostRepository = createHostRepository({ sql: trx });
                    // Act
                    try {
                        await hostRepository.getHost(expectedHostId);
                    } catch (actualError) {
                        // Assert
                        expect(actualError).toBeInstanceOf(HostNotFoundError);
                    }
                });
            });
        });
    });
});
