use crate::make_middleware_stack;
use crate::routes::health::healthz;
use axum::{Router, routing::get};
use domain::assistant::service::{AssistantService, AssistantServiceImpl};
use domain::chat::service::{
    ChatSessionService, ChatSessionServiceImpl, ChatTurnService, ChatTurnServiceImpl,
};
use infra::{
    config::Settings,
    grpc::orchestrator::client::GrpcChatOrchestratorClient,
    sqlx::assistant::repositories::SqlxAssistantRepository,
    sqlx::chat::repositories::{SqlxChatMessageRepository, SqlxChatSessionRepository},
};
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pub chat_session_service: Arc<dyn ChatSessionService + Send + Sync>,
    pub chat_turn_service: Arc<dyn ChatTurnService + Send + Sync>,
    pub assistant_service: Arc<dyn AssistantService + Send + Sync>,
}

pub struct App {
    pub addr: String,
    pub port: u16,
    router: Router,
    listener: TcpListener,
}

impl App {
    pub async fn build(settings: Settings) -> Result<Self, anyhow::Error> {
        let pool = infra::sqlx::db::get_pool(&settings.database).await?;
        let chat_session_repo = SqlxChatSessionRepository::new(pool.clone());
        let chat_message_repo = SqlxChatMessageRepository::new(pool.clone());

        let assistant_repo = SqlxAssistantRepository::new(pool);
        let assistant_service = Arc::new(AssistantServiceImpl::new(assistant_repo.clone()));

        let chat_session_service = Arc::new(ChatSessionServiceImpl::new(
            chat_session_repo.clone(),
            chat_message_repo.clone(),
        ));

        let orchestrator_client = GrpcChatOrchestratorClient::new(settings.orchestrator.endpoint)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create orchestrator client: {}", e))?;

        let chat_turn_service = Arc::new(ChatTurnServiceImpl::new(
            orchestrator_client,
            chat_session_repo,
            chat_message_repo,
            assistant_repo,
        ));

        let state = AppState {
            chat_session_service,
            chat_turn_service,
            assistant_service,
        };

        let router = Router::new()
            .route("/api/v1/healthz", get(healthz))
            .nest("/api/v1/chat", crate::routes::chat::router())
            .nest("/api/v1/assistants", crate::routes::assistants::router())
            .layer(make_middleware_stack!())
            .with_state(state);

        let listener = TcpListener::bind(format!(
            "{}:{}",
            settings.application.host, settings.application.port
        ))
        .await?;

        let addr = listener.local_addr()?.to_string();
        let port = listener.local_addr()?.port();

        let app = Self {
            addr,
            port,
            router,
            listener,
        };
        Ok(app)
    }

    pub async fn run(self) {
        println!("Listening on {}", self.addr);
        axum::serve(self.listener, self.router)
            .await
            .expect("Failed to start server");
    }
}
