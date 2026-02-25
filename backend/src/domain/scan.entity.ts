export type ScanStatus = 'scheduled' | 'in progress' | 'done';

export type ScanEntity = {
    id: string;
    project_id: string;
    created_at: Date;
    target: string;
    status: ScanStatus;
};

export type ScanInsert = Pick<ScanEntity, 'project_id' | 'target'>;
export type ScanInsertSeed = Pick<ScanEntity, 'id' | 'project_id' | 'target'>;
