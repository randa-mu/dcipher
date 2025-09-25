use axum::Router;
use axum::routing::get;
use std::net::Ipv4Addr;
use tokio::net::TcpListener;

pub struct HealthcheckServer {
    router: Router,
    listener: TcpListener,
}
impl HealthcheckServer {
    pub async fn new(ip_addr: Ipv4Addr, port: u16) -> anyhow::Result<Self> {
        let listener = TcpListener::bind((ip_addr, port)).await?;
        let router = Router::new().route("/health", get(|| async { "ok" }));

        tracing::info!(port = port, "Healthcheck server created");

        Ok(Self { listener, router })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        tracing::info!("Healthcheck server started");

        Ok(axum::serve(self.listener, self.router).await?)
    }
}
