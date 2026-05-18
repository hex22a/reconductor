#[derive(Debug, PartialEq)]
pub(crate) struct PageInfo {
    pub(crate) has_next_page: bool,
    pub(crate) end_cursor: Option<String>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Page<T> {
    pub(crate) data: Vec<T>,
    pub(crate) page_info: PageInfo,
}
