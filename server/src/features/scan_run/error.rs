#[derive(Debug)]
pub enum ScanRunError {
    NotFound,
    NoLastCursor,
}

impl From<sqlx::Error> for ScanRunError {
    fn from(_: sqlx::Error) -> Self {
        Self::NotFound
    }
}
