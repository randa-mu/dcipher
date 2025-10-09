use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

pub(crate) struct ApiServer {
    port: u16,
    app: Router<()>,
}
impl ApiServer {
    pub fn new(port: u16) -> Self {
        let app = Router::new().route("/health", get(healthcheck_handler));
        Self { port, app }
    }

    pub async fn start(self) -> anyhow::Result<()> {
        println!("Listening on port {}", self.port);
        let listener = TcpListener::bind(("0.0.0.0", self.port)).await?;
        Ok(axum::serve(listener, self.app).await?)
    }
}

async fn healthcheck_handler() -> &'static str {
    "ok"
}
