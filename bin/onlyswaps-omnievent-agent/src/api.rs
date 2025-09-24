use crate::config::ApiConfig;
use crate::state::AppState;
use axum::Router;
use axum::routing::{MethodRouter, get};
use tokio::net::TcpListener;
use tokio::sync::watch;

pub(crate) struct HttpApi {
    listener: TcpListener,
    rx: watch::Receiver<AppState>,
}

impl HttpApi {
    pub async fn new(config: &ApiConfig, rx: watch::Receiver<AppState>) -> anyhow::Result<Self> {
        let listener = TcpListener::bind(format!("{}:{}", config.hostname, config.port)).await?;
        tracing::info!(port = config.port, "API server created");
        Ok(Self { listener, rx })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        tracing::info!("API server started");
        let router = Router::new().route("/transactions", self.get_transactions());
        Ok(axum::serve(self.listener, router).await?)
    }

    fn get_transactions(&self) -> MethodRouter {
        let rx = self.rx.clone();
        get(move || {
            let state = rx.borrow().clone();
            async move { axum::Json(state) }
        })
    }
}
