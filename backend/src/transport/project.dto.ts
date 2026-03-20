import type { PageInfo } from './pageInfo';

export type ProjectDto = {
    id: string;
    name: string;
    created_at: Date;
};

export type ProjectEdge = {
    node: ProjectDto;
    cursor: string;
};

export type ProjectsDto = {
    edges: ProjectEdge[];
    pageInfo: PageInfo;
};
