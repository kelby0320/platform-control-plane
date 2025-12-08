use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatTurnRequest {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ChatTurnEventType;

impl ChatTurnEventType {
    pub const TOKEN_CHUNK: &'static str = "TokenChunk";
    pub const DONE: &'static str = "Done";
    pub const ERROR: &'static str = "Error";
}

#[derive(Debug, Serialize)]
pub struct ChatTurnTokenChunkData {
    pub content: String,
    pub is_first: bool,
    pub is_last: bool,
}
