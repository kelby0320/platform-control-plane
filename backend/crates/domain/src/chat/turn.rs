use crate::assistant::Assistant;
use crate::chat::errors::ChatTurnError;
use crate::chat::messages::ChatMessage;
use crate::chat::session::ChatSession;
use futures::Stream;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct ChatTurn {
    pub session: ChatSession,
    pub assistant: Assistant,
    pub user_message: ChatMessage,
    pub history_tail: Vec<ChatMessage>,
}

#[derive(Debug, Clone)]
pub struct TokenChunk {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct HistoryDelta {
    pub new_messages: Vec<ChatMessage>,
}

#[derive(Debug, Clone)]
pub struct Metrics {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, Clone)]
pub enum ChatEvent {
    Token(TokenChunk),
    HistoryDelta(HistoryDelta),
    Metrics(Metrics),
    Done,
    Error(String),
}

pub type ChatEventStream = Pin<Box<dyn Stream<Item = Result<ChatEvent, ChatTurnError>> + Send>>;
