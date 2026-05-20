export type PageInfo = {
    has_next_page: boolean;
    end_cursor: string;
};

export type Page<T> = {
    data: Array<T>;
    page_info: PageInfo;
};
