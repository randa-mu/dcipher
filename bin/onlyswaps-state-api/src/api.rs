use crate::config::ApiConfig;
use crate::state::AppState;
use axum::Router;
use axum::routing::{MethodRouter, get};
use std::net::SocketAddrV4;
use tokio::net::TcpListener;
use tokio::sync::watch;

pub(crate) struct HttpApi {
    listener: TcpListener,
    rx: watch::Receiver<AppState>,
}

impl HttpApi {
    pub async fn new(config: &ApiConfig, rx: watch::Receiver<AppState>) -> anyhow::Result<Self> {
        let listener = TcpListener::bind(SocketAddrV4::new(config.hostname, config.port)).await?;
        Ok(Self { listener, rx })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        let router = Router::new().route("/transactions", self.get_transactions());
        tracing::info!("API server starting");
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
