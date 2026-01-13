use crate::app::AppState;
use crate::dtos::chat::turn::{ChatTurnEventType, ChatTurnRequest, ChatTurnTokenChunkData};
use axum::{
    Json,
    extract::{Path, State},
    response::sse::{Event, Sse},
};
use domain::chat::{turn::ChatEvent, values::SessionId};
use futures::Stream;
use futures::stream::StreamExt;
use std::convert::Infallible;
use uuid::Uuid;

pub async fn new_chat_turn(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Json(request): Json<ChatTurnRequest>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let session_id = SessionId::from(session_id);

    // TODO: Get user_id from auth context.
    let user_id = state
        .chat_session_service
        .get_session(session_id.clone())
        .await
        .map_err(|_| {
            Event::default()
                .event(ChatTurnEventType::ERROR)
                .data("Session not found")
        })
        .unwrap()
        .user_id;

    let stream = state
        .chat_turn_service
        .start_turn(session_id, user_id, request.message)
        .await
        .map_err(|e| {
            Event::default()
                .event(ChatTurnEventType::ERROR)
                .data(e.to_string())
        })
        .unwrap();

    let mapped_stream = stream
        .map(|result| {
            match result {
                Ok(ChatEvent::Token(chunk)) => {
                    let data = ChatTurnTokenChunkData {
                        content: chunk.text,
                    };
                    Event::default()
                        .event(ChatTurnEventType::TOKEN_CHUNK)
                        .json_data(data)
                        .unwrap()
                }
                Ok(ChatEvent::Done) => Event::default().event(ChatTurnEventType::DONE).data("{}"),
                _ => Event::default().comment("ignored"), // Ignore other events
            }
        })
        .map(Ok);

    Sse::new(mapped_stream)
}
