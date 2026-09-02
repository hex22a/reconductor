use sqlx::error::DatabaseError;

#[derive(Debug)]
pub enum ScanResultError {
    NotFound,
    AddError(Box<dyn DatabaseError>),
    InternalError,
}

impl From<sqlx::Error> for ScanResultError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound,
            sqlx::Error::Database(e) => Self::AddError(e),
            _ => Self::InternalError,
        }
    }
}
