use chrono::{DateTime, Utc};
use domain::chat::{
    messages::ChatMessage,
    turn::{ChatEvent, HistoryDelta, Metrics, TokenChunk},
    values::{MessageId, MessageRole, SessionId},
};
use prost_types::Timestamp;
use uuid::Uuid;

use crate::grpc::orchestrator::proto::aisp::v1::{
    AssistantConfig, ChatEvent as ProtoChatEvent, ChatTurnRequest, HistoryContext, MessageEntry,
    Role as ProtoRole, UserInput, chat_event::Payload,
};

use domain::chat::turn::ChatTurn;

/// Convert domain ChatTurn to proto ChatTurnRequest
pub fn build_proto_request(turn: ChatTurn) -> ChatTurnRequest {
    let request_id = Uuid::new_v4().to_string();
    let session_id = Uuid::from(turn.session.id).to_string();
    let user_id = Uuid::from(turn.session.user_id).to_string();

    let assistant_config = AssistantConfig {
        assistant_id: Uuid::from(turn.assistant.id).to_string(),
        model_profile_id: Uuid::from(turn.assistant.model_profile_id).to_string(),
        graph_profile_id: Uuid::from(turn.assistant.graph_profile_id).to_string(),
        system_prompt: turn.assistant.system_prompt,
    };

    let history_context = HistoryContext {
        tail: turn
            .history_tail
            .into_iter()
            .map(map_message_to_proto)
            .collect(),
    };

    let user_input = UserInput {
        message: turn.user_message.content,
    };

    ChatTurnRequest {
        request_id,
        session_id,
        user_id,
        assistant: Some(assistant_config),
        history: Some(history_context),
        input: Some(user_input),
    }
}

/// Convert domain ChatMessage to proto MessageEntry
fn map_message_to_proto(message: ChatMessage) -> MessageEntry {
    MessageEntry {
        id: Uuid::from(message.id).to_string(),
        role: map_role_to_proto(message.role) as i32,
        content: message.content,
        created_at: Some(chrono_to_prost_timestamp(message.created_at)),
    }
}

/// Convert domain MessageRole to proto Role
fn map_role_to_proto(role: MessageRole) -> ProtoRole {
    match role {
        MessageRole::User => ProtoRole::User,
        MessageRole::Assistant => ProtoRole::Assistant,
        MessageRole::System => ProtoRole::System,
    }
}

/// Convert proto Role to domain MessageRole
fn map_role_from_proto(role: i32) -> Result<MessageRole, String> {
    match role {
        x if x == ProtoRole::Unspecified as i32 => Err("Unspecified role is not valid".to_string()),
        x if x == ProtoRole::User as i32 => Ok(MessageRole::User),
        x if x == ProtoRole::Assistant as i32 => Ok(MessageRole::Assistant),
        x if x == ProtoRole::System as i32 => Ok(MessageRole::System),
        _ => Err(format!("Unknown role value: {}", role)),
    }
}

/// Convert proto ChatEvent to domain ChatEvent
pub fn map_proto_event(event: ProtoChatEvent) -> Result<ChatEvent, String> {
    match event.payload {
        Some(Payload::Token(token_event)) => Ok(ChatEvent::Token(TokenChunk {
            text: token_event.content,
            is_first: token_event.is_first,
            is_last: token_event.is_last,
        })),
        Some(Payload::HistoryDelta(history_delta)) => {
            let new_messages = history_delta
                .new_messages
                .into_iter()
                .map(|entry| {
                    let role = map_role_from_proto(entry.role)?;
                    let created_at = entry
                        .created_at
                        .map(prost_timestamp_to_chrono)
                        .unwrap_or_else(Utc::now);
                    let id = Uuid::parse_str(&entry.id)
                        .map_err(|e| format!("Invalid message ID: {}", e))?;
                    Ok(ChatMessage {
                        id: MessageId::from(id),
                        session_id: SessionId::from(Uuid::nil()), // Will be set by caller if needed
                        role,
                        content: entry.content,
                        created_at,
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(ChatEvent::HistoryDelta(HistoryDelta { new_messages }))
        }
        Some(Payload::Metrics(metrics_event)) => Ok(ChatEvent::Metrics(Metrics {
            prompt_tokens: metrics_event.prompt_tokens,
            completion_tokens: metrics_event.completion_tokens,
            total_tokens: metrics_event.total_tokens,
        })),
        Some(Payload::Error(error_event)) => Ok(ChatEvent::Error(error_event.message)),
        Some(Payload::Done(_)) => Ok(ChatEvent::Done),
        None => Err("ChatEvent has no payload".to_string()),
    }
}

/// Convert chrono::DateTime<Utc> to prost_types::Timestamp
fn chrono_to_prost_timestamp(dt: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}

/// Convert prost_types::Timestamp to chrono::DateTime<Utc>
fn prost_timestamp_to_chrono(ts: Timestamp) -> DateTime<Utc> {
    DateTime::from_timestamp(ts.seconds, ts.nanos as u32).unwrap_or_else(Utc::now)
}
