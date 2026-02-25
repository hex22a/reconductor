import { describe, expect, test, it } from 'bun:test';
import { catchRollback, withTrx } from '../decorators';
import { createScanRepository, type ScanRepository } from '@/src/persistence/scan.db';
import type { ScanEntity } from '@/src/domain/scan.entity';
import { createScanFixture, expectedExistingScanId, expectedScanTarget } from '../fixtures/scans';
import { expectedExistingProjectId } from '../fixtures/projects';
import { ScanNotFoundError } from '@/src/domain/errors/ScanNotFoundError';

describe('scan.db', () => {
    test('createScanRepository', async () => {
        await catchRollback(async () => {
            await withTrx(async (trx) => {
                // Arrange
                const expectedScanRepository: ScanRepository = {
                    createScan: expect.any(Function),
                    getScan: expect.any(Function),
                };
                // Act
                const actualScanRepository: ScanRepository = createScanRepository({ sql: trx });
                // Assert
                expect(actualScanRepository).toEqual(expectedScanRepository);
            });
        });
    });

    describe('createScan', () => {
        const expectedStatus = 'scheduled';

        it('creates a scan in database', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedTarget = '192.168.0.0/16';
                    const [, expectedScanInsert] = createScanFixture(expectedExistingProjectId, expectedTarget);
                    const scanRepository: ScanRepository = createScanRepository({ sql: trx });
                    // Act
                    const actualScan: ScanEntity = await scanRepository.createScan(expectedScanInsert);
                    // Assert
                    expect(actualScan.id).toBeString();
                    expect(actualScan.created_at).toBeDate();
                    expect(actualScan.project_id).toEqual(expectedExistingProjectId);
                    expect(actualScan.target).toEqual(expectedTarget);
                    expect(actualScan.status).toEqual(expectedStatus);
                });
            });
        });

        it('throws an error when attempting to add a scan with wrong project id', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedErrorCode = '23503';
                    const expectedTarget = '192.168.0.0/16';
                    const expectedWrongProjectId = '5ffe9624-5f76-4534-b804-a569613822d0';
                    const [, expectedScanInsert] = createScanFixture(expectedWrongProjectId, expectedTarget);
                    const scanRepository: ScanRepository = createScanRepository({ sql: trx });
                    // Act
                    try {
                        await scanRepository.createScan(expectedScanInsert);
                    } catch (actualError) {
                        // Assert
                        expect(actualError).toBeInstanceOf(Bun.SQL.PostgresError);
                        expect((actualError as Bun.SQL.PostgresError).errno).toEqual(expectedErrorCode);
                    }
                });
            });
        });

        it('throws an error when attempting to add a scan with wrong target', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedErrorCode = '22P02';
                    const expectedTarget = '192.168.0.';
                    const [, expectedScanInsert] = createScanFixture(expectedExistingProjectId, expectedTarget);
                    const scanRepository: ScanRepository = createScanRepository({ sql: trx });
                    // Act
                    try {
                        await scanRepository.createScan(expectedScanInsert);
                    } catch (actualError) {
                        // Assert
                        expect(actualError).toBeInstanceOf(Bun.SQL.PostgresError);
                        expect((actualError as Bun.SQL.PostgresError).errno).toEqual(expectedErrorCode);
                    }
                });
            });
        });
    });

    describe('getScan', () => {
        it('returns a scan if exists', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedStatus = 'scheduled';
                    const scanRepository: ScanRepository = createScanRepository({ sql: trx });
                    // Act
                    const actualScan: ScanEntity = await scanRepository.getScan(expectedExistingScanId);
                    // Assert
                    expect(actualScan.id).toEqual(expectedExistingScanId);
                    expect(actualScan.created_at).toBeDate();
                    expect(actualScan.project_id).toEqual(expectedExistingProjectId);
                    expect(actualScan.target).toEqual(expectedScanTarget);
                    expect(actualScan.status).toEqual(expectedStatus);
                });
            });
        });

        it('throws an error is scan not found', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedScanId = '019c8a70-1fb0-7d53-aef0-5e163f62fb24';
                    const scanRepository: ScanRepository = createScanRepository({ sql: trx });
                    // Act
                    try {
                        await scanRepository.getScan(expectedScanId);
                    } catch (actualError) {
                        // Assert
                        expect(actualError).toBeInstanceOf(ScanNotFoundError);
                    }
                });
            });
        });
    });
});
