use crate::BalanceView;
use crate::config::AppConfig;
use anyhow::Context;
use axum::routing::get;
use axum::{Router, extract};
use extract::Extension;
use prometheus::{Encoder, GaugeVec, Opts, Registry, TextEncoder};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub(crate) struct MetricsService {
    registry: Registry,
    balance_gauge: GaugeVec,
    listen_addr: SocketAddr,
}
impl MetricsService {
    pub async fn new(config: &AppConfig) -> anyhow::Result<Self> {
        let listen_addr: SocketAddr = format!(
            "{}:{}",
            config.agent.healthcheck_listen_addr, config.agent.healthcheck_port
        )
        .parse()?;
        let registry = Registry::new();
        let balance_opts = Opts::new("balance", "wallet balances for dcipher stakeholders");
        let balance_gauge =
            GaugeVec::new(balance_opts, &["address", "chain_id", "asset", "holder"])
                .context("failed to create balance gauge")?;

        registry.register(Box::new(balance_gauge.clone()))?;

        Ok(Self {
            registry,
            balance_gauge,
            listen_addr,
        })
    }

    pub async fn serve(self: Arc<Self>) -> anyhow::Result<()> {
        let service = self.clone();
        let router = Router::new()
            .route("/health", get(|| async { "ok" }))
            .route("/metrics", get(get_metrics_handler))
            .layer(Extension(service));

        tracing::info!(
            listen_addr = self.listen_addr.to_string(),
            "starting metrics server"
        );
        let listener = TcpListener::bind(self.listen_addr).await?;
        Ok(axum::serve(listener, router).await?)
    }
    pub fn report(&self, view: BalanceView) {
        tracing::trace!("reporting balance metric");
        let address = view.address.to_string();
        let chain_id = view.chain_id.to_string();
        let asset = view.asset;
        let balance = view.balance;
        let holder = view.label;

        self.balance_gauge
            .with_label_values(&[&address, &chain_id, &asset, &holder])
            .set(balance);
    }
}

async fn get_metrics_handler(Extension(service): Extension<Arc<MetricsService>>) -> String {
    let encoder = TextEncoder::new();
    let metric_families = service.registry.gather();
    let mut buffer = vec![];
    encoder
        .encode(&metric_families, &mut buffer)
        .expect("encoding metrics failed");
    String::from_utf8(buffer).expect("metrics somehow weren't valid utf8")
}
