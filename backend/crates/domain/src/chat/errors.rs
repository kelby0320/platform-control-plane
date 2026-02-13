use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChatSessionError {
    #[error("Chat session not found")]
    NotFound,
    #[error("Session title is too long")]
    TitleTooLong,
    #[error("Missing required field: {0}")]
    MissingRequiredField(&'static str),
    #[error("Repository failure: {0}")]
    RepoFailure(String),
    #[error("Invalid message role")]
    InvalidRole,
}

#[derive(Error, Debug)]
pub enum ChatMessageError {
    #[error("Missing required field: {0}")]
    MissingRequiredField(&'static str),
}

#[derive(Error, Debug)]
pub enum ChatTurnError {
    #[error("Missing required field: {0}")]
    MissingRequiredField(&'static str),
    #[error(transparent)]
    ChatMessage(#[from] ChatMessageError),
    #[error("Orchestrator error: {0}")]
    Orchestrator(String),
    #[error("Internal error: {0}")]
    Internal(String),
}
