use crate::grpc::orchestrator::mapper::{build_proto_request, map_proto_event};
use crate::grpc::orchestrator::proto::aisp::v1::chat_orchestrator_client::ChatOrchestratorClient;
use domain::chat::errors::ChatTurnError;
use domain::chat::port::ChatOrchestratorPort;
use domain::chat::turn::{ChatEventStream, ChatTurn};
use std::sync::atomic::{AtomicU32, Ordering};
use tonic::transport::Channel;
use tracing::instrument;

pub struct GrpcChatOrchestratorClient {
    client: ChatOrchestratorClient<Channel>,
}

impl GrpcChatOrchestratorClient {
    /// Create a new gRPC client connected to the specified endpoint
    pub async fn new(endpoint: String) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let client = ChatOrchestratorClient::connect(endpoint).await?;
        Ok(Self { client })
    }

    /// Create a new gRPC client from an existing channel
    pub fn from_channel(channel: Channel) -> Self {
        Self {
            client: ChatOrchestratorClient::new(channel),
        }
    }
}

#[async_trait::async_trait]
impl ChatOrchestratorPort for GrpcChatOrchestratorClient {
    #[instrument(
        name = "grpc_chat_orchestrator_client.start_chat_turn",
        level = "INFO",
        skip_all,
        err
    )]
    async fn start_chat_turn(&self, turn: ChatTurn) -> Result<ChatEventStream, ChatTurnError> {
        // Convert domain ChatTurn to proto ChatTurnRequest
        let request = build_proto_request(turn);
        let proto_request = tonic::Request::new(request);

        // Call the gRPC service
        let mut client = self.client.clone();
        let mut stream = client
            .chat_turn(proto_request)
            .await
            .map_err(|e| ChatTurnError::Orchestrator(format!("gRPC call failed: {}", e)))?
            .into_inner();

        // Map the stream of proto events to domain events
        let event_count = AtomicU32::new(0);
        let mapped_stream = async_stream::stream! {
            loop {
                match stream.message().await {
                    Ok(Some(proto_event)) => {
                        event_count.fetch_add(1, Ordering::Relaxed);
                        match map_proto_event(proto_event) {
                            Ok(domain_event) => yield Ok(domain_event),
                            Err(e) => yield Err(ChatTurnError::Orchestrator(format!("Failed to map proto event: {}", e))),
                        }
                    }
                    Ok(None) => {
                        // Stream ended
                        let count = event_count.load(Ordering::Relaxed);
                        tracing::debug!(
                            event = "grpc_chat_orchestrator_client.start_chat_turn",
                            event_count = count
                        );
                        break;
                    }
                    Err(e) => {
                        let count = event_count.load(Ordering::Relaxed);
                        tracing::debug!(
                            event = "grpc_chat_orchestrator_client.start_chat_turn",
                            event_count = count
                        );
                        yield Err(ChatTurnError::Orchestrator(format!("gRPC stream error: {}", e)));
                        break;
                    }
                }
            }
        };

        Ok(Box::pin(mapped_stream))
    }
}
