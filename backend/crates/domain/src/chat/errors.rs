use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChatSessionError {
    #[error("Chat session not found")]
    NotFound,
}
