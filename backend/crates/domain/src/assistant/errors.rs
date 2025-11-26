use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssistantError {
    #[error("Assistant not found")]
    NotFound,
    #[error("Assistant name is too long")]
    NameTooLong,
    #[error("Repository failure: {0}")]
    RepoFailure(String),
}
