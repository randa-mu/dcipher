mod eth;
mod parsing;
mod pending;
mod signing;
mod util;

mod config;
mod signals;
mod threshold;

use crate::config::{CliConfig, load_app_config};
use crate::eth::NetworkBus;
use crate::signals::{SignalEvent, SignalManager};
use crate::threshold::create_bn254_signer;
use alloy::hex;
use anyhow::anyhow;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_config = load_app_config(&CliConfig::parse())?;
    let network_bus = NetworkBus::create(&app_config.networks).await?;
    let pending_verifications = network_bus.fetch_pending_verifications().await?;
    let signer = create_bn254_signer(&app_config, network_bus)?;

    for verification in pending_verifications {
        let signature = signer.try_sign(verification).await?;

        println!("signing a pending verification: {}", hex::encode(signature))
    }

    let signals = SignalManager::new(
        app_config.agent.healthcheck_listen_addr,
        app_config.agent.healthcheck_port,
    )
    .await?;
    println!("Listening on port {}", app_config.agent.healthcheck_port);
    tokio::select! {
        signal = signals.next() => {
            match signal {
                SignalEvent::SigTerm | SignalEvent::SigInt | SignalEvent::CtrlC => Ok(()),
                SignalEvent::HealthcheckServerFailed => Err(anyhow!("axum stopped unexpectedly"))
            }
        }
    }
}
