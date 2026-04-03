export type ScanStatus = 'scheduled' | 'in progress' | 'done';

export type ScanEntity = {
    id: string;
    project_id: string;
    created_at: Date;
    target: string;
    status: ScanStatus;
    schedule: string | null;
    next_run_at: Date | null;
};

export type ScanInsert = Pick<ScanEntity, 'project_id' | 'target' | 'schedule' | 'next_run_at'>;
export type ScanInsertSeed = Pick<
    ScanEntity,
    'id' | 'project_id' | 'target' | 'schedule' | 'next_run_at'
>;
