use sqlx::error::DatabaseError;

#[derive(Debug)]
pub enum ScanError {
    NotFound,
    UpdateError(Box<dyn DatabaseError>),
    InternalError,
}

impl From<sqlx::Error> for ScanError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound,
            sqlx::Error::Database(e) => Self::UpdateError(e),
            _ => Self::InternalError,
        }
    }
}
