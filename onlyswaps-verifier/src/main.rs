mod eth;
mod parsing;
mod pending;
mod signing;
mod util;

mod config;
mod signals;
mod threshold;
mod transport;

use crate::config::{CliConfig, load_app_config};
use crate::eth::NetworkBus;
use crate::signals::{SignalEvent, SignalManager};
use crate::signing::ChainService;
use crate::threshold::create_bn254_signer;
use crate::transport::create_libp2p_transport;
use anyhow::anyhow;
use clap::Parser;
use std::sync::Arc;
use std::time::Duration;

const REFRESH_INTERVAL: Duration = Duration::from_secs(5);
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_config = load_app_config(&CliConfig::parse())?;
    let network_bus = Arc::new(NetworkBus::create(&app_config.networks).await?);
    let transport = create_libp2p_transport(&app_config)?;
    println!("libp2p transport created");
    let signer = create_bn254_signer(&app_config, Arc::clone(&network_bus), transport)?;
    println!("threshold signer created");

    tokio::task::spawn(async move {
        loop {
            tokio::time::interval(REFRESH_INTERVAL).tick().await;
            let pending_verifications = network_bus
                .fetch_pending_verifications()
                .await
                .unwrap_or_default();
            for verification in pending_verifications {
                println!(
                    "processing pending verification: {}",
                    verification.request_id
                );
                let verified_swap = signer.try_sign(&verification).await.unwrap();
                println!("message signed");
                match network_bus
                    .as_ref()
                    .submit_verification(verification.chain_id, verified_swap)
                    .await
                {
                    Ok(_) => println!("completed a swap on chain {}", verification.chain_id),
                    Err(e) => eprintln!("error submitting verification: {}", e),
                }
            }
        }
    });

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
