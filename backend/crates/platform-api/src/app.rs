use crate::routes::health::healthz;
use axum::{Router, routing::get};
use domain::chat::service::{ChatSessionService, ChatSessionServiceImpl};
use infra::{
    config::Settings, sqlx::chat::repositories::SqlxChatMessageRepository,
    sqlx::chat::repositories::SqlxChatSessionRepository,
};
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pub chat_session_service: Arc<dyn ChatSessionService + Send + Sync>,
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
        let chat_message_repo = SqlxChatMessageRepository::new(pool);
        let chat_session_service = Arc::new(ChatSessionServiceImpl::new(
            chat_session_repo,
            chat_message_repo,
        ));

        let state = AppState {
            chat_session_service,
        };

        let router = Router::new()
            .route("/api/v1/healthz", get(healthz))
            .nest("/api/v1/chat", crate::routes::chat::router())
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
