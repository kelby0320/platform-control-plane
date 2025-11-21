use sqlx::Error;

pub struct SqlxError(Error);

impl From<Error> for SqlxError {
    fn from(error: Error) -> Self {
        Self(error)
    }
}

impl From<SqlxError> for Error {
    fn from(error: SqlxError) -> Self {
        error.0
    }
}
