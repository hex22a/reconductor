import type { PageInfo } from './pageInfo';

export type ScanDto = {
    id: string;
    created_at: Date;
    target: string;
    schedule?: string;
};

export type ScanEdge = {
    node: ScanDto;
    cursor: string;
};

export type ScansDto = {
    edges: ScanEdge[];
    pageInfo: PageInfo;
};
