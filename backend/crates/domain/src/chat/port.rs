use crate::chat::errors::ChatTurnError;
use crate::chat::turn::{ChatEventStream, ChatTurn};
use async_trait::async_trait;

#[async_trait]
pub trait ChatOrchestratorPort {
    async fn start_chat_turn(&self, turn: ChatTurn) -> Result<ChatEventStream, ChatTurnError>;
}
