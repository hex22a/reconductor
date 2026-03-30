export type CreateScanDto = {
    target: string;
    projectId: string;
    schedule?: string;
};

export type ScanDto = {
    id: string;
    created_at: Date;
    target: string;
    status: string;
    schedule?: string;
};
