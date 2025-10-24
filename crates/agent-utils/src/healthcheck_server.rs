use axum::Router;
use axum::handler::Handler;
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

    pub fn with_metrics<H, T>(mut self, get_handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        let router = std::mem::take(&mut self.router);
        self.router = router.route("/metrics", get(get_handler));
        self
    }

    pub async fn start(self) -> anyhow::Result<()> {
        tracing::info!("Healthcheck server started");

        Ok(axum::serve(self.listener, self.router).await?)
    }
}
