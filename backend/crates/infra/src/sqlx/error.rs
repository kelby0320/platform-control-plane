use sqlx::Error;

use std::fmt;

#[derive(Debug)]
pub struct SqlxError(Error);

impl fmt::Display for SqlxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for SqlxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

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
