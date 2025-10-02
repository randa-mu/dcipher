use crate::cli::StartArgs;
use crate::config::{AppConfig, AppConfigFile};
use crate::eth::NetworkBus;
use crate::events::{EventManagement, create_omnievent_management};
use crate::pending::{RequestId, Verification};
use crate::signing::{DsignerWrapper, OnlySwapsSigner};
use crate::threshold::create_bn254_signer;
use crate::transport::create_libp2p_transport;
use agent_utils::healthcheck_server::HealthcheckServer;
use agent_utils::monitoring::init_monitoring;
use alloy::providers::DynProvider;
use alloy::signers::local::PrivateKeySigner;
use ark_bn254::Bn254;
use config::file::load_mapped_config_file;
use dcipher_signer::bls::BlsPairingSigner;
use futures::StreamExt;
use std::sync::Arc;

pub async fn start_daemon(args: StartArgs) -> anyhow::Result<()> {
    let config = load_mapped_config_file::<AppConfigFile, AppConfig>(args.config)?;
    let healthcheck_server = HealthcheckServer::new(
        config.agent.healthcheck_listen_addr,
        config.agent.healthcheck_port,
    )
    .await?;
    init_monitoring(&config.agent)?;

    // listen for OS signals or any of the tasks closing and shut down either gracefully
    // or noisily with errors
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;
    tokio::select! {
        _ = sigterm.recv() => Ok(()),

        _ = sigint.recv() => Ok(()),

        _ = tokio::signal::ctrl_c() => Ok(()),

        res = healthcheck_server.start() =>  {
           match res {
                Ok(()) => anyhow::bail!("healthcheck stopped unexpectedly without an error"),
                Err(e) => anyhow::bail!("healthcheck stopped unexpectedly: {}", e),
           }
        }

        res = run_onlyswaps(&config) => {
           match res {
                Ok(()) => anyhow::bail!("swap loop stopped unexpectedly without an error"),
                Err(e) => anyhow::bail!("swap loop stopped unexpectedly: {}", e),
           }
        }
    }
}

async fn run_onlyswaps(config: &AppConfig) -> anyhow::Result<()> {
    let signer = PrivateKeySigner::from_slice(config.eth_private_key.as_slice())?;
    let network_bus = Arc::new(NetworkBus::new(signer, &config.networks).await?);
    let transport =
        create_libp2p_transport(&config.longterm_secret.libp2p_sk, &config.committee_config)?;
    let dsigner = create_bn254_signer(
        config.listen_addr.clone(),
        &config.committee_config,
        transport,
    )?;
    let signer =
        OnlySwapsSigner::<NetworkBus<DynProvider>, DsignerWrapper<BlsPairingSigner<Bn254>>>::new(
            network_bus.clone(),
            dsigner,
        );
    tracing::info!(
        multiaddr = config.listen_addr.to_string(),
        n = config.committee_config.n,
        t = config.committee_config.t,
        "threshold signer created"
    );

    // all contract events and load-on-start state are ingested via a channel and
    // they are serialised out of the channel into the signer. This is actually asyncified once
    // popped off the channel so that different nodes don't get stuck signing different
    // messages when e.g. in catchup
    let (tx, mut rx_verifications) =
        tokio::sync::mpsc::unbounded_channel::<Verification<RequestId>>();

    // start listening for new contract events and write them to the channel
    let EventManagement {
        event_ids,
        omnievent,
    } = create_omnievent_management(&config.networks).await?;
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
    let pending_verifications = network_bus
        .clone()
        .fetch_pending_verifications()
        .await
        .unwrap_or_default();
    for verification in pending_verifications {
        tx.send(verification)?;
    }

    // loop over all the futures, be that incoming verification events or OS shutdown signals.
    // the swap evaluation has a timeout to stop a broken RPC or malformed request
    // blocking the world forever
    while let Some(verification) = rx_verifications.recv().await {
        tracing::info!(
            chain_id = verification.chain_id,
            request_id = verification.request_id.to_string(),
            "attempting verification"
        );
        let signer = signer.clone();
        tokio::spawn(async move {
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
        });
    }
    Ok(())
}
