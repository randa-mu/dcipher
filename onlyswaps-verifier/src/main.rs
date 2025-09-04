mod eth;
mod parsing;
mod pending;
mod signing;
mod util;

mod config;
mod config_network;
mod events;
mod monitoring;
mod signals;
mod threshold;
mod transport;

use crate::config::{CliConfig, load_app_config};
use crate::eth::NetworkBus;
use crate::events::{EventManagement, create_omnievent_management};
use crate::monitoring::init_monitoring;
use crate::pending::{RequestId, Verification};
use crate::signals::{SignalEvent, SignalManager};
use crate::signing::OnlySwapsSigner;
use crate::threshold::create_bn254_signer;
use crate::transport::create_libp2p_transport;
use clap::Parser;
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load config files and set up all the plumbing
    let app_config = load_app_config(&CliConfig::parse())?;
    init_monitoring(&app_config)?;

    let network_bus = NetworkBus::create(&app_config.networks).await?;
    let transport = create_libp2p_transport(&app_config)?;
    tracing::info!(
        multiaddr = app_config.libp2p.multiaddr.to_string(),
        "libp2p transport created"
    );

    let dsigner = create_bn254_signer(&app_config, transport)?;
    let signer = OnlySwapsSigner::new(&network_bus, &dsigner);
    tracing::info!(
        n = app_config.committee.n,
        t = app_config.committee.t,
        "threshold signer created"
    );

    // all contract events and load-on-start state are ingested via a channel and
    // we serialise writing back to the contracts to simplify things like nonce management,
    // profitability calculations, etc
    let (tx, mut rx_verifications) =
        tokio::sync::mpsc::unbounded_channel::<Verification<RequestId>>();

    // start listening for new contract events and write them to the channel
    let EventManagement {
        event_ids,
        omnievent,
    } = create_omnievent_management(&app_config.networks).await?;
    let mut stream = omnievent.get_ethereum_multi_event_stream(event_ids).await?;
    let tx1 = tx.clone();
    tokio::spawn(async move {
        while let Some(Ok(event)) = stream.next().await {
            let verification = event.data.try_into().expect("invalid event received");
            tx1.send(verification)
                .expect("failed to send verification on channel");
        }
    });

    // but also fetch any outstanding verifications that existed before our node started so we
    // can verify them too (and maybe help an existing running node to aggregate a signature, as we
    // may have been offline for other partials).
    let tx = tx.clone();
    let network_bus = NetworkBus::create(&app_config.networks).await?;
    let pending_verifications = network_bus
        .fetch_pending_verifications()
        .await
        .unwrap_or_default();
    for verification in pending_verifications {
        tx.send(verification)?;
    }

    // start listening for app signals so we can gracefully shutdown while processing
    // pending verifications
    let signals = SignalManager::new(
        app_config.agent.healthcheck_listen_addr,
        app_config.agent.healthcheck_port,
    )
    .await?;
    let shutdown = signals.next();
    tokio::pin!(shutdown);

    // loop over all the futures, be that incoming verification events or OS shutdown signals.
    // the swap evaluation has a timeout to stop a broken RPC or malformed request
    // blocking the world forever
    loop {
        tokio::select! {
            next_verification = rx_verifications.recv() => {
                if let Some(verification) = next_verification {
                    tracing::info!(
                        chain_id = verification.chain_id,
                        request_id = verification.request_id.to_string(),
                        "attempting verification"
                    );
                    match signer.evaluate_and_send(&verification).await {
                        Err(e) => {
                            tracing::error!(
                                chain_id = verification.chain_id,
                                request_id = verification.request_id.to_string(),
                                error = e.to_string(),
                                "verification returned an error"
                            );
                        }
                        Ok(_) => {
                            tracing::info!(
                                chain_id = verification.chain_id,
                                request_id = verification.request_id.to_string(),
                                "verification completed successfully"
                            );
                        }
                    }
                }
            }
            shutdown_signal = &mut shutdown => {
                match shutdown_signal {
                    SignalEvent::SigTerm | SignalEvent::SigInt | SignalEvent::CtrlC => anyhow::bail!("finished"),
                    SignalEvent::HealthcheckServerFailed => anyhow::bail!("axum stopped unexpectedly"),
                }
            }
        }
    }
}
