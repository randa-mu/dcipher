use axum::Router;
use axum::routing::get;
use std::net::Ipv4Addr;
use tokio::net::TcpListener;

#[derive(Debug, Clone, Copy)]
pub enum SignalEvent {
    SigTerm,
    SigInt,
    CtrlC,
    HealthcheckServerFailed,
}
pub struct SignalManager {
    healthcheck: Router,
    sigterm: tokio::signal::unix::Signal,
    sigint: tokio::signal::unix::Signal,
    listener: TcpListener,
}
impl SignalManager {
    pub async fn new(ip_addr: Ipv4Addr, port: u16) -> anyhow::Result<Self> {
        let listener = TcpListener::bind((ip_addr, port)).await?;
        let healthcheck = Router::new().route("/health", get(|| async { "ok" }));

        Ok(Self {
            healthcheck,
            sigterm: tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?,
            sigint: tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?,
            listener,
        })
    }

    pub async fn next(mut self) -> SignalEvent {
        tokio::select! {
            _ = self.sigterm.recv() => SignalEvent::SigTerm,
            _ = self.sigint.recv()  => SignalEvent::SigInt,
            _ = tokio::signal::ctrl_c() => SignalEvent::CtrlC,
            _ = axum::serve(self.listener, self.healthcheck) => SignalEvent::HealthcheckServerFailed,
        }
    }
}
