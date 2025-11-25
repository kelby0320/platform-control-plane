use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChatSessionError {
    #[error("Chat session not found")]
    NotFound,
    #[error("Session title is too long")]
    TitleTooLong,
    #[error("Repository failure: {0}")]
    RepoFailure(String),
    #[error("Invalid message role")]
    InvalidRole,
}
