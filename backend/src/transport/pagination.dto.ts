import type { PageInfo } from './pageInfo.dto';

export type Pagination<T> = {
    edges: T[];
    pageInfo: PageInfo;
};
