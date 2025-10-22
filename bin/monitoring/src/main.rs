use crate::config::{AppConfig, CliArgs};
use crate::metrics::MetricsService;
use crate::probe::ProbeService;
use ::config::file::load_config_file;
use agent_utils::monitoring::init_monitoring;
use alloy::primitives::Address;
use clap::Parser;
use std::sync::Arc;

mod config;
mod maths;
mod metrics;
mod probe;
mod provider;

pub struct BalanceView {
    address: Address,
    label: String,
    asset: String,
    balance: f64,
    chain_id: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli_args = CliArgs::parse();
    let config: AppConfig = load_config_file(cli_args.config_path)?;

    init_monitoring(&config.agent)?;

    let metrics = MetricsService::new(&config).await?;
    let metrics = Arc::new(metrics);

    let probe = ProbeService::new(&config).await?;
    let mut ticker = tokio::time::interval(config.metrics.frequency);

    tokio::spawn({
        let metrics = metrics.clone();
        async move {
            loop {
                let balances = probe.fetch_balances().await;
                for balance in balances {
                    metrics.report(balance);
                }
                ticker.tick().await;
            }
        }
    });

    metrics.serve().await
}
