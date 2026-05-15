#[derive(Debug, PartialEq)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub page_info: PageInfo,
}
