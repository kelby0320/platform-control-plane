use domain::chat::ChatSessionError;
use sqlx::Error;

pub fn from_sqlx_error(error: Error) -> ChatSessionError {
    ChatSessionError::RepoFailure(error.to_string())
}
