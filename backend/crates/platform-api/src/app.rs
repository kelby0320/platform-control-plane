use crate::config::Settings;
use crate::routes::health::healthz;
use axum::{Router, routing::get};

pub struct App {
    addr: String,
    router: Router,
}

impl App {
    pub async fn build(settings: Settings) -> Result<Self, std::io::Error> {
        let router = Router::new().route("/api/v1/healthz", get(healthz));

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
