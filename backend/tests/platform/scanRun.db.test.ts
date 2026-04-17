import { describe, expect, test, it } from 'bun:test';
import { catchRollback, withTrx } from '../decorators';
import { createScanRunRepository, type ScanRunRepository } from '@/src/persistence/scanRun';
import { expectedExistingScanId } from '../fixtures/scans';
import type { ScanRunEntity } from '@/src/domain/scanRun.entity';
import { ScanRunNotFoundError } from '@/src/domain/errors/ScanRunNotFoundError';
import { expectedExistingScanRunId } from '../fixtures/scanRuns';

describe('scanRun.db', () => {
    test('createScanRunRepository', async () => {
        await catchRollback(async () => {
            await withTrx(async (trx) => {
                // Arrange
                const expectedScanRunRepository: ScanRunRepository = {
                    getScanRun: expect.any(Function),
                    listScanRuns: expect.any(Function),
                };
                // Act
                const actualScanRepository: ScanRunRepository = createScanRunRepository({
                    sql: trx,
                });
                // Assert
                expect(actualScanRepository).toEqual(expectedScanRunRepository);
            });
        });
    });

    describe('getScanRun', () => {
        it('returns a scan run if exists', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const scanRunRepository: ScanRunRepository = createScanRunRepository({
                        sql: trx,
                    });
                    // Act
                    const actualScanRun: ScanRunEntity =
                        await scanRunRepository.getScanRun(expectedExistingScanRunId);
                    // Assert
                    expect(actualScanRun.id).toEqual(expectedExistingScanRunId);
                    expect(actualScanRun.scan_id).toEqual(expectedExistingScanId);
                    expect(actualScanRun.created_at).toBeDate();
                });
            });
        });

        it('throws an error is scan not found', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const expectedScanRunId = '019c8a70-1fb0-7d53-aef0-5e163f62fb24';
                    const scanRepository: ScanRunRepository = createScanRunRepository({ sql: trx });
                    // Act
                    try {
                        await scanRepository.getScanRun(expectedScanRunId);
                    } catch (actualError) {
                        // Assert
                        expect(actualError).toBeInstanceOf(ScanRunNotFoundError);
                    }
                });
            });
        });
    });

    describe('listScans', () => {
        it('returns list of all scan runs for a given scan', async () => {
            await catchRollback(async () => {
                await withTrx(async (trx) => {
                    // Arrange
                    const scanRepository: ScanRunRepository = createScanRunRepository({ sql: trx });
                    // Act
                    const { scanRuns, hasNextPage } =
                        await scanRepository.listScanRuns(expectedExistingScanId);
                    // Assert
                    expect(scanRuns).toHaveLength(1);
                    expect(hasNextPage).toBeFalse();
                });
            });
        });
    });
});
