use crate::common::spawn_app;
use futures::StreamExt;
use platform_api::dtos::chat::turn::ChatTurnRequest;
use reqwest::Client;
use reqwest_eventsource::{Event, EventSource};
use serde_json::json;

mod common;

#[tokio::test]
async fn test_new_chat_turn() {
    let app = spawn_app().await;
    let client = Client::new();

    // 1. Create a session
    let response = client
        .post(format!("{}/api/v1/chat/sessions", app.address))
        .json(&json!({
            "title": "Test Session",
            "assistant_id": "733750f6-66bb-4365-abcc-7ee1e989b339"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 200);
    let session_json: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    let session_id = session_json["id"].as_str().expect("Missing id");

    // 2. Send a turn request
    let turn_request = ChatTurnRequest {
        message: "Hello".to_string(),
    };

    let request = client
        .post(format!(
            "{}/api/v1/chat/sessions/{}/turns",
            app.address, session_id
        ))
        .json(&turn_request);

    let mut event_source = EventSource::new(request).expect("Failed to create EventSource");

    let mut token_chunks = Vec::new();
    let mut done_received = false;

    while let Some(event) = event_source.next().await {
        match event {
            Ok(Event::Message(message)) => {
                let message_json: serde_json::Value =
                    serde_json::from_str(&message.data).unwrap_or(serde_json::Value::Null);

                match message.event.as_str() {
                    "TokenChunk" => {
                        let content = message_json["content"].as_str().expect("Missing content");
                        token_chunks.push(content.to_string());
                    }
                    "Done" => {
                        done_received = true;
                        break;
                    }
                    _ => {}
                }
            }
            Err(reqwest_eventsource::Error::StreamEnded) => {
                break;
            }
            Err(e) => panic!("Event source error: {:?}", e),
            _ => {}
        }
    }

    assert!(done_received, "Did not receive Done event");
    assert_eq!(token_chunks, vec!["hello", " world", "!"]);

    event_source.close();
}
