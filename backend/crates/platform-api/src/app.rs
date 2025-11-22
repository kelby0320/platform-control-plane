use crate::routes::health::healthz;
use axum::{Router, routing::get};
use domain::chat::service::{ChatSessionService, ChatSessionServiceImpl};
use infra::{config::Settings, sqlx::chat::repositories::SqlxChatSessionRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub chat_session_service: Arc<dyn ChatSessionService + Send + Sync>,
}

pub struct App {
    addr: String,
    router: Router,
}

impl App {
    pub async fn build(settings: Settings) -> Result<Self, anyhow::Error> {
        let pool = infra::sqlx::db::get_pool(&settings.database).await?;
        let chat_session_repo = SqlxChatSessionRepository::new(pool);
        let chat_session_service = Arc::new(ChatSessionServiceImpl::new(chat_session_repo));

        let state = AppState {
            chat_session_service,
        };

        let router = Router::new()
            .route("/api/v1/healthz", get(healthz))
            .nest("/api/v1/chat", crate::routes::chat::router())
            .with_state(state);

        let addr = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );
        let app = Self { addr, router };
        Ok(app)
    }

    pub async fn run(self) {
        let listener = tokio::net::TcpListener::bind(&self.addr)
            .await
            .expect("Failed to bind address");

        println!("Listening on {}", self.addr);
        axum::serve(listener, self.router)
            .await
            .expect("Failed to start server");
    }
}
