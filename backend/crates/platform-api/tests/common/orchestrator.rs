use async_stream::try_stream;
use futures::Stream;
use infra::grpc::orchestrator::proto::aisp::v1::chat_orchestrator_server::{
    ChatOrchestrator, ChatOrchestratorServer,
};
use infra::grpc::orchestrator::proto::aisp::v1::{
    ChatEvent, ChatTurnRequest, DoneEvent, TokenChunkEvent, chat_event,
};
use std::pin::Pin;
use tokio::net::TcpListener;
use tonic::{Request, Response, Status, transport::Server};

pub struct MockChatOrchestrator;

#[tonic::async_trait]
impl ChatOrchestrator for MockChatOrchestrator {
    type ChatTurnStream = Pin<Box<dyn Stream<Item = Result<ChatEvent, Status>> + Send + 'static>>;

    async fn chat_turn(
        &self,
        _request: Request<ChatTurnRequest>,
    ) -> Result<Response<Self::ChatTurnStream>, Status> {
        // Simple dummy response: "hello", " world", "!" then Done.
        let output = try_stream! {
            let chunks = vec!["hello", " world", "!"];
            for chunk in chunks {
                yield ChatEvent {
                    payload: Some(chat_event::Payload::Token(
                        TokenChunkEvent {
                            content: chunk.to_string(),
                        }
                    ))
                };
            }
             yield ChatEvent {
                payload: Some(chat_event::Payload::Done(
                    DoneEvent {}
                ))
            };
        };

        Ok(Response::new(Box::pin(output) as Self::ChatTurnStream))
    }
}

pub async fn start_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        Server::builder()
            .add_service(ChatOrchestratorServer::new(MockChatOrchestrator))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
            .await
            .unwrap();
    });

    format!("http://{}", addr)
}
